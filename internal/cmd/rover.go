package cmd

import (
	"errors"
	"strconv"
	"strings"
)

type Orientation rune
type Direction rune

var InvalidOrientation = errors.New("invalid orientation")
var InvalidDirection = errors.New("invalid direction")

const (
	North = Orientation('N')
	East  = Orientation('E')
	South = Orientation('S')
	West  = Orientation('W')

	Forward = Direction('F')
	Left    = Direction('L')
	Right   = Direction('R')
)

type Rover interface {
	Move(direction Direction) (x, y int8, err error)
	Orientation() Orientation
	Reverse() (x, y int8)
}

type rover struct {
	x, y        int8
	orientation Orientation
}

func (r rover) Orientation() Orientation {
	return r.orientation
}

func (r *rover) Move(direction Direction) (x, y int8, err error) {
	if (direction != Left) && (direction != Right) && (direction != Forward) {
		return 0, 0, InvalidDirection
	}

	if direction == Left {
		switch r.orientation {
		case North:
			r.orientation = West
		case West:
			r.orientation = South
		case South:
			r.orientation = East
		case East:
			r.orientation = North
		}
		return r.x, r.y, err
	}

	if direction == Right {
		switch r.orientation {
		case North:
			r.orientation = East
		case East:
			r.orientation = South
		case South:
			r.orientation = West
		case West:
			r.orientation = North
		}
		return r.x, r.y, err
	}

	switch r.orientation {
	case North:
		r.y++
	case East:
		r.x++
	case South:
		r.y--
	case West:
		r.x--
	}
	return r.x, r.y, err
}

func (r *rover) Reverse() (x, y int8) {
	switch r.orientation {
	case North:
		r.y--
	case East:
		r.x--
	case South:
		r.y++
	case West:
		r.x++
	}
	return r.x, r.y
}

func NewRover(x, y int8, orientation Orientation) Rover {
	return &rover{x: x, y: y, orientation: orientation}
}

func NewRoverFromString(input string) (r Rover, err error) {
	split := strings.Split(input, " ")
	parsedX, err := strconv.ParseUint(split[0], 10, 8)
	if err != nil {
		return nil, err
	}
	parsedY, err := strconv.ParseUint(split[1], 10, 8)
	if err != nil {
		return nil, err
	}
	var orientation Orientation
	switch split[2] {
	case "N":
		orientation = North
	case "E":
		orientation = East
	case "S":
		orientation = South
	case "W":
		orientation = West
	default:
		return nil, InvalidOrientation
	}
	return NewRover(int8(parsedX), int8(parsedY), orientation), nil
}
