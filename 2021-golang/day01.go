package adventofcode

import (
	"bufio"
	"io"
	"strconv"
)

func DayOnePartOne(input io.Reader) (int, error) {
	nums, err := parseNumbers(input)
	if err != nil {
		return 0, err
	}

	return calculateNumberOfIncreases(nums), nil
}

func DayOnePartTwo(input io.Reader) (int, error) {
	nums, err := parseNumbers(input)
	if err != nil {
		return 0, err
	}

	nums = slideWindowOfThree(nums)

	return calculateNumberOfIncreases(nums), nil
}

func calculateNumberOfIncreases(input []int) int {
	var count int
	var previous int
	for _, current := range input {
		if previous != 0 && current > previous {
			count++
		}

		previous = current
	}

	return count
}

func slideWindowOfThree(nums []int) []int {
	windowed := make([]int, 0)

	for i := 0; i <= len(nums) - 3; i++ {
		windowed = append(windowed, nums[i] + nums[i+1] + nums[i+2])
	}

	return windowed
}

// ParseNumbers read r trying to parse an int by each non-empty line.
func parseNumbers(r io.Reader) ([]int, error)  {
	scanner := bufio.NewScanner(r)

	nums := make([]int, 0)

	for scanner.Scan() {
		str := scanner.Text()
		if str == "" {
			continue
		}
		num, err := strconv.Atoi(str)
		if err != nil {
			return nil, err
		}

		nums = append(nums, num)
	}

	return nums, nil
}