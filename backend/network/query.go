package network

import (
	"douban_movie/backend/model"
	"douban_movie/package/logger"
	"encoding/json"
	"io"
	"math/rand"
	"net/http"
	"strings"

	"github.com/klauspost/compress/gzip"
)

func OnQuery(name string) ([]model.QueryResult, error) {
	req, err := http.NewRequest("GET", "https://movie.douban.com/j/subject_suggest?q="+name, nil)
	if err != nil {
		logger.Warn("query request error", err)
	}
	req.Header.Set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.51")
	req.Header.Set("Accept", "*/*")
	req.Header.Set("Accept-Encoding", "gzip")
	req.Header.Set("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6,zh-TW;q=0.5")
	req.Header.Set("Connection", "keep-alive")
	//req.Header.Set("Host", "https://movie.douban.com/")
	req.AddCookie(&http.Cookie{Name: "bid", Value: genBid()})
	client := &http.Client{}
	res, err := client.Do(req)
	if err != nil {
		logger.Warn("query response error", err)
		return nil, err
	}
	//buf, _ := io.ReadAll(res.Body)
	reader, err := gzip.NewReader(res.Body)
	defer res.Body.Close()
	if err != nil {
		logger.Warn("query unzip error", err)
		return nil, err
	}

	buf, _ := io.ReadAll(reader)

	results := make([]model.QueryResult, 0)
	err = json.Unmarshal(buf, &results)
	if err != nil {
		logger.Warn("query unmarshal error", err)
		return nil, err
	}
	return results, nil
}

const charset = "123456789-_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

func genBid() string {
	sb := strings.Builder{}
	sb.Grow(11)
	for i := 0; i < 11; i++ {
		sb.WriteByte(charset[rand.Intn(len(charset))])
	}
	return sb.String()
}
