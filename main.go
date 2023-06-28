package main

import (
	"douban_movie/package/logger"
	"douban_movie/package/util"
	"embed"
	"math/rand"
	"time"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
	"github.com/wailsapp/wails/v2/pkg/options/windows"
)

//go:embed all:frontend/dist
var assets embed.FS

func main() {
	// Create an instance of the app structure
	rand.NewSource(time.Now().UnixNano())
	app := NewApp()
	util.CreateDirNotExists("data/webview")
	// Create application with options
	err := wails.Run(&options.App{
		Title:  "",
		Width:  512,
		Height: 384,
		AssetServer: &assetserver.Options{
			Assets: assets,
		},
		Frameless:         true,
		HideWindowOnClose: true,
		BackgroundColour:  &options.RGBA{R: 255, G: 255, B: 255, A: 1},
		OnStartup:         app.startup,
		Bind: []interface{}{
			app,
		},
		Windows: &windows.Options{
			DisableFramelessWindowDecorations: false,
			//DisableWindowIcon:   true,
			WebviewUserDataPath: "data/webview",
		},
	})

	if err != nil {
		logger.Error("Error:", err.Error())
	}
}
