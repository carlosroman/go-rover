package cmd

import (
	"bufio"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRun(t *testing.T) {
	input := `5 5
5 3
1 1 E
RFRFRFRF
`
	t.Log(input)
	s:= strings.NewReader(input)
	reader := bufio.NewReader(s)
	err := Run(reader)
	assert.NoError(t, err)
}
