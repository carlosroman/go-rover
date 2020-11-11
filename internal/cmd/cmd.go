package cmd

import (
	"bufio"
	"fmt"
	"io"
	"strings"

	"github.com/carlosroman/go-rover/internal/mars"
)

func Run(reader *bufio.Reader, out io.Writer) error {

	err, m := getMap(reader)
	if err != nil {
		return err
	}

	for {
		_, err := reader.Peek(1)
		if err != nil {
			if err == io.EOF {
				return nil
			}
			return err
		}
		err, r := getRover(reader)
		if err != nil {
			return err
		}
		if err = move(r, m, reader, out); err != nil {
			return err
		}
	}
}

func move(rover Rover, mars mars.Mars, reader *bufio.Reader, out io.Writer) error {
	text, err := reader.ReadString('\n')
	if err != nil {
		return err
	}
	var x, y int8
	var lost string
	for i := range trimSuffix(text) {
		x, y = rover.Move(Direction(text[i]))
		if !mars.IsValidLocation(uint8(x), uint8(y)) {
			lost = "LOST"
		}
	}

	_, err = fmt.Fprintln(out, strings.TrimSpace(
		fmt.Sprintf("%v %v %v %v", x, y, string(rover.Orientation()), lost)))
	return err
}

func getRover(reader *bufio.Reader) (error, Rover) {
	text, err := reader.ReadString('\n')
	if err != nil {
		return err, nil
	}

	err, r := NewRoverFromString(trimSuffix(text))
	if err != nil {
		return err, nil
	}
	return nil, r

}

func getMap(reader *bufio.Reader) (error, mars.Mars) {
	text, err := reader.ReadString('\n')
	if err != nil {
		return err, nil
	}
	err, m := mars.NewMars(trimSuffix(text))
	if err != nil {
		return err, nil
	}
	return nil, m
}

func trimSuffix(text string) string {
	return strings.TrimSuffix(text, "\n")
}
