package config

import (
	"fmt"
	"github.com/spf13/viper"
	"os"
)

type Configuration struct {
	DatabaseID string `json:"database_id"`
	Token      string `json:"token"`
}

var Conf *Configuration
var wd string

func init() {
	viper.AddConfigPath("data/keys")
	viper.SetConfigType("json")
	Conf = new(Configuration)
	wd, _ = os.Getwd()
	err := os.Mkdir(wd+"/data", os.ModePerm)
	if err != nil {
		fmt.Println(err)
	}
	err = os.Mkdir(wd+"/data/keys", os.ModePerm)
	if err != nil {
		fmt.Println(err)
	}
}

func LoadConfig() []string {
	//	search for files in current directory
	dir, err := os.ReadDir(wd + "/data/keys")
	if err != nil {
		return nil
	}
	var files []string
	for _, f := range dir {
		if !f.IsDir() {
			files = append(files, f.Name())
		}
	}
	return files
}

func CreateConfig(configName string, id string, token string) error {
	viper.Set("database_id", id)
	viper.Set("token", token)
	return viper.WriteConfigAs(wd + "/data/keys/" + configName + ".json")
}

func ReadConfig(configName string) {
	viper.SetConfigName(configName)
	err := viper.ReadInConfig() // Find and read the config file
	if err != nil {             // Handle errors reading the config file
		panic(fmt.Errorf("请检查配置文件是否正确: %w", err))
	}
	Conf.DatabaseID = viper.GetString("database_id")
	Conf.Token = viper.GetString("token")
}
