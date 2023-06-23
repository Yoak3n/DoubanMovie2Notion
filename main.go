package main

import (
	"embed"
	"fmt"
	"github.com/wailsapp/wails/v2/pkg/options/windows"
	"os"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
)

//go:embed all:frontend/dist
var assets embed.FS

func main() {
	// Create an instance of the app structure
	app := NewApp()
	wd, _ := os.Getwd()
	err := os.Mkdir(wd+"/data/webview", os.ModePerm)
	if err != nil {
		fmt.Println(err)
	}
	// Create application with options
	err = wails.Run(&options.App{
		Title:  "豆瓣电影入库",
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
			WebviewUserDataPath: "./data/webview",
		},
	})

	if err != nil {
		println("Error:", err.Error())
	}
}
