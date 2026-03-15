package main

import (
	"context"
	"douban_movie/backend/model"
	"douban_movie/backend/network"
	"douban_movie/config"
	"douban_movie/package/logger"
	"fmt"
	"io"
)

// App struct
type App struct {
	ctx context.Context
}
type Result struct {
	Name   string `json:"name"`
	Status bool   `json:"status"`
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
}

// LoadOptions loads options from config
func (a *App) LoadOptions() (options []string) {
	loadConfig := config.LoadConfig()
	options = append(loadConfig, "创建新配置")
	return
}

func (a *App) AppRun(target string, option string) Result {
	config.ReadConfig(option)
	crawl, err := network.NewCrawl(target)
	if err != nil {
		logger.Error(err)
		return Result{err.Error(), false}
	} else {
		movie := crawl.FetchMovie()
		logger.INFO.Printf("fetch movie data: %+v\n", movie)

		if movie.Name == "" {
			return Result{"无法获取到电影信息，可能是被豆瓣暂时限制访问，请稍后再试或检查日志", false}
		}

		res := network.Post2Notion(network.MakePost(movie))
		if res == nil {
			return Result{"上传 Notion 失败，请检查网络连接或 API 配置", false}
		}

		result := &Result{
			Name: movie.Name,
		}

		if res.StatusCode == 200 {
			result.Status = true
			return *result
		} else {
			result.Status = false
			buf, err := io.ReadAll(res.Body)
			if err != nil {
				result.Name = err.Error()
			} else {
				defer res.Body.Close()
				result.Name = string(buf)
			}
			return *result
		}
	}

}

func (a *App) WriteConfig(configName string, id string, token string) string {
	err := config.CreateConfig(configName, id, token)
	if err != nil {
		return err.Error()
	}
	return ""
}

// QueryMovie queries movie info
func (a *App) QueryMovie(name string) []model.QueryResult {
	results, err := network.OnQuery(name)
	if err != nil {
		logger.Warn(err)
		return results
	}
	return results
}

// Greet returns a greeting for the given name
func (a *App) Greet(name string) string {
	return fmt.Sprintf("Hello %s, It's show time!", name)
}
