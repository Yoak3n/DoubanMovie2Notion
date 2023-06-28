package util

import "os"

func CreateDirNotExists(dir string) {
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		e := os.Mkdir(dir, 0755)
		if e != nil {
			return
		}
	}
}
