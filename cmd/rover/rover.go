package main

import (
	"bufio"
	"fmt"
	"os"

	"github.com/carlosroman/go-rover/internal/cmd"
)

func main() {
	reader := bufio.NewReader(os.Stdin)
	if err := cmd.Run(reader, os.Stdout); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
