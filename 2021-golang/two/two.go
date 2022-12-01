package two

import (
	"bufio"
	"errors"
	"io"
	"strconv"
	"strings"
)

var ErrInvalidAction = errors.New("invalid action")

type command struct {
	action action
	num    int
}

type action int

const (
	forward action = iota
	down
	up
)

type submarine struct {
	position int
	depth    int
}

func (s *submarine) execute(cmd command) {
	switch cmd.action {
	case forward:
		s.position += cmd.num
		break
	case down:
		s.depth += cmd.num
		break
	case up:
		s.depth -= cmd.num
		break
	}
}

type advancedSubmarine struct {
	submarine
	aim int
}

func (a *advancedSubmarine) execute(cmd command) {
	switch cmd.action {
	case forward:
		a.position += cmd.num
		a.depth += a.aim * cmd.num
		break
	case down:
		a.aim += cmd.num
		break
	case up:
		a.aim -= cmd.num
		break
	}
}

func CalculatePositionAndDepth(input io.Reader) (int, error) {
	commands, err := parseCommands(input)
	if err != nil {
		return 0, err
	}

	sub := &submarine{}

	for _, cmd := range commands {
		sub.execute(cmd)
	}

	return sub.position * sub.depth, nil
}

func CalculateAdvancedPositionAndDepth(input io.Reader) (int, error) {
	commands, err := parseCommands(input)
	if err != nil {
		return 0, err
	}

	sub := &advancedSubmarine{}

	for _, cmd := range commands {
		sub.execute(cmd)
	}

	return sub.position * sub.depth, nil
}

func parseCommands(r io.Reader) ([]command, error) {
	scanner := bufio.NewScanner(r)

	commands := make([]command, 0, 10)

	for scanner.Scan() {
		str := scanner.Text()
		if str == "" {
			continue
		}

		cmd := command{}
		var err error

		a, b, _ := strings.Cut(str, " ")

		cmd.num, err = strconv.Atoi(b)
		if err != nil {
			return nil, err
		}

		switch a {
		case "forward":
			cmd.action = forward
			break
		case "up":
			cmd.action = up
			break
		case "down":
			cmd.action = down
			break
		default:
			return nil, ErrInvalidAction
		}

		commands = append(commands, cmd)
	}

	return commands, nil
}
