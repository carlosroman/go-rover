package cmd

import (
	"bufio"
	"bytes"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRun_Okay(t *testing.T) {
	input := `5 3
1 1 E
RFRFRFRF
`
	t.Log(input)
	s := strings.NewReader(input)
	reader := bufio.NewReader(s)
	buf := &bytes.Buffer{}
	err := Run(reader, buf)
	assert.NoError(t, err)
	actual := string(buf.Bytes())
	assert.Equal(t, "1 1 E\n", actual)
}

func TestRun_Lost(t *testing.T) {
	input := `5 3
3 2 N
FRRFLLFFRRFLL
`
	t.Log(input)
	s := strings.NewReader(input)
	reader := bufio.NewReader(s)
	buf := &bytes.Buffer{}
	err := Run(reader, buf)
	assert.NoError(t, err)
	actual := string(buf.Bytes())
	assert.Equal(t, "3 3 N LOST\n", actual)
}

func TestRun_Other(t *testing.T) {
	input := `5 3
0 3 W
LLFFFLFLFL
`
	t.Log(input)
	s := strings.NewReader(input)
	reader := bufio.NewReader(s)
	buf := &bytes.Buffer{}
	err := Run(reader, buf)
	assert.NoError(t, err)
	actual := string(buf.Bytes())
	assert.Equal(t, "2 4 S LOST\n", actual)
}

func TestRun_RoverIgnoresLostCommands(t *testing.T) {
	input := `5 3
1 1 E
RFRFRFRF
3 2 N
FRRFLLFFRRFLL
0 3 W
LLFFFLFLFL
`
	t.Log(input)
	s := strings.NewReader(input)
	reader := bufio.NewReader(s)
	buf := &bytes.Buffer{}
	err := Run(reader, buf)
	assert.NoError(t, err)
	actual := string(buf.Bytes())
	assert.Equal(t, "1 1 E\n3 3 N LOST\n2 3 S\n", actual)
}
