package logger

import (
	"io"
	"log"
	"os"
)

var (
	INFO  *log.Logger
	DEBUG *log.Logger
	WARN  *log.Logger
	ERROR *log.Logger
)

func init() {
	wd, _ := os.Getwd()
	err := os.Mkdir(wd+"/data/logs", os.ModePerm)
	if err != nil {
		log.Println(err)
	}
	file, err := os.OpenFile("data/logs/errors.log",
		os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0666)
	if err != nil {
		log.Fatalln("Can't open file error.log:", err)
	}
	defer file.Close()
	INFO = log.New(log.Writer(), "[INFO] ", log.LstdFlags|log.Lmsgprefix|log.Lshortfile)
	DEBUG = log.New(log.Writer(), "[DEBUG] ", log.LstdFlags|log.Lmsgprefix|log.Lshortfile)
	WARN = log.New(log.Writer(), "[WARN] ", log.LstdFlags|log.Lmsgprefix|log.Lshortfile)
	ERROR = log.New(io.MultiWriter(file, log.Writer()), "[ERROR] ", log.LstdFlags|log.Lmsgprefix|log.Lshortfile)
}
func Info(v ...interface{}) {
	INFO.Println(v[:])
}
func Debug(v ...interface{}) {
	DEBUG.Println(v[:])
}
func Warn(v ...interface{}) {
	WARN.Println(v[:])
}
func Error(v ...interface{}) {
	ERROR.Println(v[:])
}
