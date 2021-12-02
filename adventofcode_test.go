package adventofcode_test

import (
	"embed"
	"io/fs"
)

//go:embed testdata
var data embed.FS

func mustOpen(name string) fs.File {
	file, err := data.Open("testdata/one-input.txt")
	if err != nil {
		panic(err)
	}

	return file
}