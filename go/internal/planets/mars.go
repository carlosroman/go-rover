package planets

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

var (
	InvalidUpperRightErr = errors.New("invalid upper right")
)

const maxMars = 50

func NewMars(input string) (err error, planet Planet) {
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

	return nil, &mars{
		x:         uint8(parsedX),
		y:         uint8(parsedY),
		markStore: make(map[string]struct{}),
	}
}

type mars struct {
	x, y      uint8
	markStore map[string]struct{}
}

func (p mars) UpperRight() (x, y uint8) {
	return p.x, p.y
}

func (p mars) IsValidLocation(x, y uint8) (valid bool) {
	if x <= p.x && y <= p.y {
		return true
	}
	return false
}

func (p *mars) MarkLocation(x, y uint8) (successful bool) {
	key := fmt.Sprintf("%v,%v", x, y)
	if _, ok := p.markStore[key]; ok {
		return false
	}
	p.markStore[key] = struct{}{}
	return true
}
