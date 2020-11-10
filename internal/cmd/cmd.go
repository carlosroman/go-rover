package cmd

import (
	"bufio"
	"fmt"
)

func Run(reader *bufio.Reader) error {
	text, err := reader.ReadString('\n')
	if err!=nil{
		return err
	}
	fmt.Printf(text)
	return nil
}
