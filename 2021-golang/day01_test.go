package adventofcode_test

import (
	"adventofcode"
	"io"
	"testing"
)

func Test_DayOne(t *testing.T) {
	input := mustOpen("testdata/one-input.txt")
	partOneExpected := 1676
	partTwoExpected := 1706

	defer func(c io.Closer) {
		_ = c.Close()
	}(input)

	partOne, err := adventofcode.DayOnePartOne(input)
	if err != nil {
		t.Fatalf("%v", err)
	}

	if partOne != partOneExpected {
		t.Fatalf("normal increment %d is not the same as expected %d", partOne, partOneExpected)
	}

	s := input.(io.Seeker)
	_,_ = s.Seek(0, io.SeekStart)

	partTwo, err := adventofcode.DayOnePartTwo(input)
	if err != nil {
		t.Fatalf("%v", err)
	}

	if partTwo != partTwoExpected {
		t.Fatalf("normal increment %d is not the same as expected %d", partTwo, partTwoExpected)
	}
}