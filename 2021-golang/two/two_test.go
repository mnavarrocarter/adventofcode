package two_test

import (
	"adventofcode/two"
	"io"
	"os"
	"testing"
)

func Test_ItSolvesPartOne(t *testing.T) {
	input := mustOpen("input.txt")
	want := 2036120

	defer func(c io.Closer) {
		_ = c.Close()
	}(input)

	got, err := two.CalculatePositionAndDepth(input)
	if err != nil {
		t.Fatalf("%v", err)
	}

	if want != got {
		t.Fatalf("got %d but wanted %d", got, want)
	}
}

func Test_ItSolvesPartTwo(t *testing.T) {
	input := mustOpen("input.txt")
	want := 2015547716

	defer func(c io.Closer) {
		_ = c.Close()
	}(input)

	got, err := two.CalculateAdvancedPositionAndDepth(input)
	if err != nil {
		t.Fatalf("%v", err)
	}

	if want != got {
		t.Fatalf("got %d but wanted %d", got, want)
	}
}

func mustOpen(path string) *os.File {
	f, err := os.Open(path)
	if err != nil {
		panic(err)
	}

	return f
}
