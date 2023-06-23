package main

import (
	"context"
	"douban_movie/backend/network"
	"douban_movie/config"
	"douban_movie/util"
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
		return Result{err.Error(), false}
	} else {
		movie := crawl.FetchMovie()
		res := network.Post2Notion(util.MakePost(movie))
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

func (a *App) WriteConfig(configname string, id string, token string) string {
	err := config.CreateConfig(configname, id, token)
	if err != nil {
		return err.Error()
	}
	return ""
}

// Greet returns a greeting for the given name
func (a *App) Greet(name string) string {
	return fmt.Sprintf("Hello %s, It's show time!", name)
}
