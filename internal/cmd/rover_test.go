package cmd_test

import (
	"fmt"
	"testing"

	"github.com/carlosroman/go-rover/internal/cmd"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func Test_RoverMove(t *testing.T) {
	startingPoint := int8(5)
	tests := []struct {
		name                 string
		startingOrientation  cmd.Orientation
		move                 cmd.Direction
		expectedX, expectedY int8
		expectedOrientation  cmd.Orientation
	}{
		{name: "Start N, move Left", startingOrientation: cmd.North, move: cmd.Left, expectedX: 5, expectedY: 5, expectedOrientation: cmd.West},
		{name: "Start N, move Right", startingOrientation: cmd.North, move: cmd.Right, expectedX: 5, expectedY: 5, expectedOrientation: cmd.East},
		{name: "Start N, move Forward", startingOrientation: cmd.North, move: cmd.Forward, expectedX: 5, expectedY: 6},
		{name: "Start E, move Forward", startingOrientation: cmd.East, move: cmd.Forward, expectedX: 6, expectedY: 5},
		{name: "Start S, move Forward", startingOrientation: cmd.South, move: cmd.Forward, expectedX: 5, expectedY: 4},
		{name: "Start W, move Forward", startingOrientation: cmd.West, move: cmd.Forward, expectedX: 4, expectedY: 5},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			r := cmd.NewRover(startingPoint, startingPoint, test.startingOrientation)
			actualX, actualY := r.Move(test.move)
			assert.Equal(t, test.expectedX, actualX)
			assert.Equal(t, test.expectedY, actualY)
			if test.expectedOrientation > 0 {
				assert.Equal(t, test.expectedOrientation, r.Orientation())
			} else {
				assert.Equal(t, test.startingOrientation, r.Orientation())
			}
		})
	}
}

func Test_RoverTurnLeft(t *testing.T) {
	startingPoint := int8(5)
	r := cmd.NewRover(startingPoint, startingPoint, cmd.North)
	for i, expected := range []cmd.Orientation{cmd.West, cmd.South, cmd.East, cmd.North} {
		_, _ = r.Move(cmd.Left)
		actual := r.Orientation
		require.Equal(t, expected, actual(), fmt.Sprintf("Expected[%v]: %s but got %s", i, string(expected), string(actual())))
	}
}

func Test_RoverTurnRight(t *testing.T) {
	startingPoint := int8(5)
	r := cmd.NewRover(startingPoint, startingPoint, cmd.North)
	for i, expected := range []cmd.Orientation{cmd.East, cmd.South, cmd.West, cmd.North} {
		_, _ = r.Move(cmd.Right)
		actual := r.Orientation
		require.Equal(t, expected, actual(), fmt.Sprintf("Expected[%v]: %s but got %s", i, string(expected), string(actual())))
	}
}

func Test_NewRover(t *testing.T) {
	tests := []struct {
		name                 string
		expectedX, expectedY int8
		expectedOrientation  cmd.Orientation
	}{
		{name: "4 1 N", expectedX: 4, expectedY: 1, expectedOrientation: cmd.North},
		{name: "3 2 E", expectedX: 3, expectedY: 2, expectedOrientation: cmd.East},
		{name: "2 3 S", expectedX: 2, expectedY: 3, expectedOrientation: cmd.South},
		{name: "1 4 W", expectedX: 1, expectedY: 4, expectedOrientation: cmd.West},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			err, r := cmd.NewRoverFromString(test.name)
			require.NoError(t, err)
			assert.Equal(t, test.expectedOrientation, r.Orientation())
			// rotate to get current location
			x, y := r.Move(cmd.Left)
			assert.Equal(t, test.expectedX, x)
			assert.Equal(t, test.expectedY, y)
		})
	}

	t.Run("InvalidOrientation", func(t *testing.T) {
		err, _ := cmd.NewRoverFromString("4 1 X")
		require.Error(t, err)
	})
}
