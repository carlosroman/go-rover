package mars

import (
	"errors"
	"strconv"
	"strings"
)

var (
	InvalidUpperRightErr = errors.New("invalid upper right")
)

const maxMars = 50

type Mars interface {
	UpperRight() (x, y uint8)
	IsValidLocation(x, y uint8) bool
}

func NewMars(input string) (err error, mars Mars) {
	split := strings.Split(input, " ")

	parsedX, err := strconv.ParseUint(split[0], 10, 8)
	if err != nil {
		return err, nil
	}
	if parsedX > maxMars {
		return InvalidUpperRightErr, nil
	}

	parsedY, err := strconv.ParseUint(split[1], 10, 8)
	if err != nil {
		return err, nil
	}
	if parsedY > maxMars {
		return InvalidUpperRightErr, nil
	}

	return nil, &planet{
		x: uint8(parsedX),
		y: uint8(parsedY),
	}
}

type planet struct {
	x, y uint8
}

func (p planet) UpperRight() (x, y uint8) {
	return p.x, p.y
}

func (p planet) IsValidLocation(x, y uint8) (valid bool) {
	if x <= p.x && y <= p.y {
		return true
	}
	return false
}
