package one_test

import (
	"adventofcode/one"
	"io"
	"os"
	"testing"
)

func mustOpen(path string) *os.File {
	f, err := os.Open(path)
	if err != nil {
		panic(err)
	}

	return f
}

func Test_CountDepthIncrease(t *testing.T) {
	input := mustOpen("input.txt")
	want := 1676

	defer func(c io.Closer) {
		_ = c.Close()
	}(input)

	got, err := one.CountDepthIncrease(input)
	if err != nil {
		t.Fatalf("%v", err)
	}

	if want != got {
		t.Fatalf("normal increment %d is not the same as expected %d", got, want)
	}
}

func Test_CountDepthIncreaseSlidingWindow(t *testing.T) {
	input := mustOpen("input.txt")
	want := 1706

	defer func(c io.Closer) {
		_ = c.Close()
	}(input)

	got, err := one.CountDepthIncreaseSlidingWindow(input)
	if err != nil {
		t.Fatalf("%v", err)
	}

	if got != want {
		t.Fatalf("normal increment %d is not the same as expected %d", got, want)
	}
}
