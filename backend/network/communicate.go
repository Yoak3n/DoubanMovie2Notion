package network

import (
	"douban_movie/backend/model"
	"douban_movie/package/logger"
	"fmt"
	"net/http"
	"regexp"
	"strconv"
	"strings"

	"github.com/PuerkitoBio/goquery"
	"github.com/antchfx/htmlquery"
)

type Crawl struct {
	target string
}

func NewCrawl(target string) (*Crawl, error) {
	crawl := new(Crawl)
	if strings.HasPrefix(target, "https://movie.douban.com/subject/") {
		crawl.target = target
	} else {
		if _, err := strconv.Atoi(target); err == nil {
			fmtTarget := fmt.Sprintf("https://movie.douban.com/subject/%s/", target)
			crawl.target = fmtTarget
		} else {
			return nil, err
		}
	}
	return crawl, nil

}

func (c *Crawl) getHtml() *goquery.Document {
	req, err := http.NewRequest("GET", c.target, nil)
	if err != nil {
		logger.Error("failed to create request: ", err)
		return nil
	}

	// 模拟真实浏览器请求头
	req.Header.Set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.51")
	req.Header.Set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
	req.Header.Set("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6")
	req.Header.Set("Referer", "https://movie.douban.com/")
	req.Header.Set("Connection", "keep-alive")

	// 添加 bid cookie 绕过一些简单的限制
	req.AddCookie(&http.Cookie{Name: "bid", Value: genBid()})

	client := &http.Client{}
	res, err := client.Do(req)
	if err != nil {
		logger.Error("request failed: ", err)
		return nil
	}
	defer res.Body.Close()

	if res.StatusCode != 200 {
		logger.Error("douban returned status code: ", res.StatusCode)
		return nil
	}

	dom, err := goquery.NewDocumentFromReader(res.Body)
	if err != nil {
		logger.Error("failed to parse html: ", err)
		return nil
	}

	// 检查是否成功解析到关键内容，如果没有，可能是被反爬了（返回了验证码页面）
	if dom.Find("span[property=\"v:itemreviewed\"]").Length() == 0 {
		htmlContent, _ := dom.Html()
		if strings.Contains(htmlContent, "验证码") || strings.Contains(htmlContent, "captcha") {
			logger.Error("detected captcha or anti-crawl page")
		} else {
			logger.Error("could not find movie name, possibly structure changed or access denied")
		}
		// 可以考虑记录前 500 个字符进行调试
		limit := 500
		if len(htmlContent) < limit {
			limit = len(htmlContent)
		}
		logger.INFO.Printf("HTML snippet: %s\n", htmlContent[:limit])
	}

	return dom
}

func (c *Crawl) FetchMovie() *model.DoubanMovie {
	logger.INFO.Printf("fetch movie data: %s\n", c.target)
	movie := &model.DoubanMovie{
		Language:    [5]string{},
		Region:      [5]string{},
		Director:    [5]string{},
		Writer:      [5]string{},
		Actor:       [5]string{},
		Genre:       [5]string{},
		Score:       0.0,
		RankNo:      "",
		ReleaseTime: "",
	}
	dom := c.getHtml()
	if dom == nil {
		logger.Error("get html error: dom is nil")
		return movie
	}
	// 电影名
	logger.INFO.Println("name")
	movie.Name = dom.Find("span[property=\"v:itemreviewed\"]").Text()
	// 导演
	logger.INFO.Printf("director: %s\n", dom.Find("a[rel=\"v:directedBy\"]").Text())
	dom.Find("a[rel=\"v:directedBy\"]").Each(func(i int, s *goquery.Selection) {
		if i < 5 {
			t := s.Text()
			if t == "" {
				t = " "
			}
			movie.Director[i] = t
		}
	})
	// 编剧
	logger.INFO.Println("writer")
	logger.INFO.Printf("writer: %s\n", dom.Find("#info>span:nth-of-type(2)>span:nth-of-type(2)>a").Text())
	dom.Find("#info>span:nth-of-type(2)>span:nth-of-type(2)>a").Each(func(i int, s *goquery.Selection) {
		if i < 5 {
			t := s.Text()
			if t == "" {
				t = " "
			}
			movie.Writer[i] = t
		}
	})
	// 主演
	logger.INFO.Println("actor")
	dom.Find("a[rel=\"v:starring\"]").Each(func(i int, s *goquery.Selection) {
		if i < 5 {
			t := s.Text()
			if t == "" {
				t = " "
			}
			movie.Actor[i] = t
		}
	})
	// 上映时间
	logger.INFO.Println("release time")
	logger.INFO.Printf("release time: %s\n", dom.Find("span[property=\"v:initialReleaseDate\"]").Text())
	movie.ReleaseTime = strings.TrimSpace(dom.Find("span[property=\"v:initialReleaseDate\"]").Text())
	// 评分
	logger.INFO.Println("score")
	t := dom.Find("strong[class=\"ll rating_num\"]").Text()
	var ft float64
	ft, err := strconv.ParseFloat(strings.TrimSpace(t), 64)
	if err != nil {
		logger.Error(err)
		ft = 0.0
	}
	fmStr := fmt.Sprintf("%.1f", ft)
	ft64, err := strconv.ParseFloat(fmStr, 64)
	movie.Score = float32(ft64)

	// 所在榜单和排名
	logger.INFO.Println("rank")
	rankLi := dom.Find(".top250-link>a").Text()
	if rankLi != "" {
		no := strings.TrimSpace(dom.Find("span[class=\"top250-no\"]").Text())
		no = strings.ReplaceAll(no, "No.", "")
		movie.RankNo = no
	}
	// 地区
	logger.INFO.Println("region")
	infoSelection := dom.Find("#info")
	if infoSelection.Length() == 0 {
		logger.Error("get info element error: #info not found")
		return movie
	}
	s := infoSelection.Get(0)
	logger.INFO.Printf("info element: %v\n", s)

	rn := htmlquery.Find(s, "//span[./text()=\"制片国家/地区:\"]/following::text()[1]")
	logger.INFO.Printf("region length %d\n", len(rn))
	if len(rn) <= 0 {
		r := "暂无"
		movie.Region[0] = r
	} else {
		r := htmlquery.InnerText(rn[0])
		rs := strings.Split(r, "/")
		for index, item := range rs {
			if index < 5 {
				if item == "" {
					item = " "
				}
				movie.Region[index] = strings.TrimSpace(item)
			}
		}
	}
	// 语种
	logger.INFO.Println("language")
	ln := htmlquery.Find(s, "//span[@class=\"pl\" and text()=\"语言:\"]/following-sibling::text()[1]")
	if len(ln) == 0 {
		l := "暂无"
		movie.Language[0] = l
	} else {
		l := htmlquery.InnerText(ln[0])
		ls := strings.Split(l, "/")
		for index, item := range ls {
			if index < 5 {
				if item == "" {
					item = " "
				}
				movie.Language[index] = strings.TrimSpace(item)
			}
		}
	}

	// 类型
	logger.INFO.Println("genre")
	dom.Find("span[property=\"v:genre\"]").Each(func(i int, s *goquery.Selection) {
		if i < 5 {
			e := s.Text()
			if e == "" {
				e = " "
			}
			movie.Genre[i] = e
		}
	})
	// 时长
	logger.INFO.Println("duration")
	runtimeSelection := dom.Find("span[property=\"v:runtime\"]")
	if runtimeSelection.Length() > 0 {
		val, _ := runtimeSelection.Attr("content")
		d, err := strconv.Atoi(val)
		if err != nil {
			logger.Error("get duration error: ", err)
			movie.Duration = 0
		} else {
			movie.Duration = d
		}
	}
	// 年份
	logger.INFO.Println("year")
	yearSelection := dom.Find("span[class=\"year\"]")
	if yearSelection.Length() > 0 {
		y := yearSelection.Text()
		reg, err := regexp.Compile("-?\\d+\\.?\\d*")
		if err != nil {
			logger.Error("compile regex error: ", err)
		} else {
			ys := reg.FindAllString(y, -1)
			if len(ys) > 0 {
				atoi, err := strconv.Atoi(strings.Join(ys, ""))
				if err != nil {
					logger.Error("get year error: ", err)
				} else {
					movie.Year = uint(atoi)
				}
			}
		}
	}
	// IMDB号
	logger.INFO.Println("imdb")
	imdb := htmlquery.Find(s, "//span[./text()=\"IMDb:\"]/following::text()[1]")
	var i string
	if len(imdb) == 0 {
		i = "暂无"
	} else {
		i = htmlquery.InnerText(imdb[0])
	}
	movie.Imdb = strings.TrimSpace(i)
	// 封面
	logger.INFO.Println("cover")
	val, _ := dom.Find("#mainpic>a>img").Attr("src")
	movie.Src = val
	fmt.Printf("已成功获取到电影《%s》的信息\n", movie.Name)
	return movie
}

func Post2Notion(req *http.Request) *http.Response {
	client := &http.Client{}
	res, err := client.Do(req)
	if err != nil {
		logger.Error("post to notion error: ", err)
		return nil
	}
	return res
}
