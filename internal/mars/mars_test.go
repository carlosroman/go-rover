package mars_test

import (
	"testing"

	"github.com/carlosroman/go-rover/internal/mars"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestNewMars(t *testing.T) {
	tests := []struct {
		name                 string
		input                string
		expectedX, expectedY uint8
		expectedErr          error
	}{
		{name: "simple", input: "5 5", expectedX: 5, expectedY: 5},
		{name: "invalidX", input: "51 5", expectedErr: mars.InvalidUpperRightErr},
		{name: "invalidY", input: "6 51", expectedErr: mars.InvalidUpperRightErr},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			err, m := mars.NewMars(test.input)
			if test.expectedErr != nil {
				require.Error(t, err)
				return
			}

			assert.NoError(t, err)
			actualX, actualY := m.UpperRight()
			assert.Equal(t, test.expectedX, actualX)
			assert.Equal(t, test.expectedY, actualY)
		})
	}
}

func TestPlanet_IsValidLocation(t *testing.T) {
	err, m := mars.NewMars("5 5")
	require.NoError(t, err)
	tests := []struct {
		name          string
		x, y          uint8
		expectedValid bool
	}{
		{"middle", 3, 3, true},
		{"validTopRight", 5, 5, true},
		{"validBottomRight", 5, 0, true},
		{"validBottomLeft", 0, 0, true},
		{"validTopLeft", 0, 5, true},
		{"invalidX", 10, 3, false},
		{"invalidX", 3, 10, false},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			isValid := m.IsValidLocation(test.x, test.y)
			assert.Equal(t, test.expectedValid, isValid)
		})
	}
}
