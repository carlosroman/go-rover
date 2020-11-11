package cmd

import (
	"errors"
	"strconv"
	"strings"
)

type Orientation rune
type Direction rune

var InvalidOrientation = errors.New("invalid Orientation")

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
	Move(direction Direction) (x, y int8)
	Orientation() Orientation
}

type rover struct {
	x, y        int8
	orientation Orientation
}

func (r rover) Orientation() Orientation {
	return r.orientation
}

func (r *rover) Move(direction Direction) (x, y int8) {
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
		return r.x, r.y
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
		return r.x, r.y
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
	return r.x, r.y
}

func NewRover(x, y int8, orientation Orientation) Rover {
	return &rover{x: x, y: y, orientation: orientation}
}

func NewRoverFromString(input string) (err error, r Rover) {
	split := strings.Split(input, " ")
	parsedX, err := strconv.ParseUint(split[0], 10, 8)
	if err != nil {
		return err, nil
	}
	parsedY, err := strconv.ParseUint(split[1], 10, 8)
	if err != nil {
		return err, nil
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
		return InvalidOrientation, nil
	}
	return nil, NewRover(int8(parsedX), int8(parsedY), orientation)
}
