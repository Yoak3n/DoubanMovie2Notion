package network

import (
	"douban_movie/backend/model"
	"douban_movie/package/logger"
	"fmt"
	"github.com/PuerkitoBio/goquery"
	"github.com/antchfx/htmlquery"
	"log"
	"net/http"
	"regexp"
	"strconv"
	"strings"
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
		panic(err)
	}
	req.Header.Set("User-Agent", `'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36'`)
	client := &http.Client{}
	res, err := client.Do(req)
	if err != nil {
		logger.Error("")
		panic(err)
	}
	dom, err := goquery.NewDocumentFromReader(res.Body)
	if err != nil {
		logger.Error("请求发送错误：", err)
		panic(err)
	}
	res.Body.Close()
	return dom
}

func (c *Crawl) FetchMovie() *model.DoubanMovie {
	var movie = new(model.DoubanMovie)
	dom := c.getHtml()
	// 电影名
	movie.Name = dom.Find("span[property=\"v:itemreviewed\"]").Text()
	// 导演
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
	movie.ReleaseTime = strings.TrimSpace(dom.Find("span[property=\"v:initialReleaseDate\"]").Text())
	// 评分
	t := dom.Find("strong[property=\"v:average\"]").Text()
	ft, err := strconv.ParseFloat(strings.TrimSpace(t), 64)
	if err != nil {
		fmt.Println("get score error")
	}
	fmStr := fmt.Sprintf("%.1f", ft)
	ft64, err := strconv.ParseFloat(fmStr, 64)
	movie.Score = float32(ft64)

	// 所在榜单和排名
	rankLi := dom.Find(".top250-link>a").Text()
	if rankLi != "" {
		no := strings.TrimSpace(dom.Find("span[class=\"top250-no\"]").Text())
		no = strings.Replace(no, "No.", "", -1)
		movie.RankNo = no
	}
	// 地区
	s := dom.Find("#info").Get(0)
	r := htmlquery.InnerText(htmlquery.Find(s, "//span[./text()=\"制片国家/地区:\"]/following::text()[1]")[0])
	rs := strings.Split(r, "/")
	for index, item := range rs {
		if index < 5 {
			if item == "" {
				item = " "
			}
			movie.Region[index] = strings.TrimSpace(item)
		}
	}
	// 语种
	l := htmlquery.InnerText(htmlquery.Find(s, "//span[@class=\"pl\" and text()=\"语言:\"]/following-sibling::text()[1])")[0])
	ls := strings.Split(l, "/")
	for index, item := range ls {
		if index < 5 {
			if item == "" {
				item = " "
			}
			movie.Language[index] = strings.TrimSpace(item)
		}
	}
	// 类型
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
	val, _ := dom.Find("span[property=\"v:runtime\"]").Attr("content")
	d, err := strconv.Atoi(val)
	if err != nil {
		panic("get duration error")
	} else {
		movie.Duration = d
	}
	// 年份
	y := dom.Find("span[class=\"year\"]").Text()
	reg, err := regexp.Compile("-?\\d+\\.?\\d*")
	if err != nil {
		panic(err)
	}
	ys := reg.FindAllString(y, -1)
	atoi, err := strconv.Atoi(strings.Join(ys, ""))
	if err != nil {
		panic("get year error")
	}
	movie.Year = uint(atoi)
	// IMDB号
	i := htmlquery.InnerText(htmlquery.Find(s, "//span[./text()=\"IMDb:\"]/following::text()[1]")[0])
	movie.Imdb = strings.TrimSpace(i)
	// 封面
	val, _ = dom.Find("#mainpic>a>img").Attr("src")
	movie.Src = val
	fmt.Printf("已成功获取到电影《%s》的信息\n", movie.Name)
	return movie
}

func Post2Notion(req *http.Request) *http.Response {
	client := &http.Client{}
	res, err := client.Do(req)
	if err != nil {
		log.Println(err)
	}
	return res
}
