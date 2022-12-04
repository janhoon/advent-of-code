package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	args := os.Args
	if len(args) != 2 {
		fmt.Println("Please provide a input file")
		os.Exit(1)
	}

	file, err := os.Open(args[1])
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer file.Close()

	total := 0
	scanner := bufio.NewScanner(file)
	pbCounter := 0
	bps := make([]string, 3)
	for scanner.Scan() {
		content := scanner.Text()
		// challenge 1
		// b1, b2 := content[:len(content)/2], content[len(content)/2:]
		// char := getFirstRepeatedCharacter(b1, b2)

		// challange 2
		if pbCounter == 2 {
			bps[pbCounter] = content
			char := getFirstThreeRepeatChar(bps[0], bps[1], bps[2])
			pbCounter = 0
			val := getCharValue(char)
			total = total + val
			fmt.Printf("char: %s points: %d total: %d\n", string(char), val, total)
			bps = make([]string, 3)
		} else {
			bps[pbCounter] = content
			pbCounter++
		}
	}

	if scanner.Err() != nil {
		fmt.Println(scanner.Err())
		os.Exit(1)
	}

	fmt.Printf("Total value of double packed: %d\n", total)
}

func getFirstRepeatedCharacter(b1 string, b2 string) rune {
	found := make(map[string]bool)
	for _, c := range b1 {
		found[string(c)] = true
	}

	for _, c := range b2 {
		if found[string(c)] {
			return c
		}
	}
	return '0'
}
func getFirstThreeRepeatChar(b1 string, b2 string, b3 string) rune {
	found := make(map[string]int)
	for _, c := range b1 {
		found[string(c)] = 1
	}

	for _, c := range b2 {
		_, ok := found[string(c)]
		if ok {
			found[string(c)] = 2
		}
	}

	for _, c := range b3 {
		val, ok := found[string(c)]
		if ok && val == 2 {
			return c
		}
	}
	return '0'
}

func getCharValue(char rune) int {
	if char >= 65 && char <= 90 {
		return int(char - 38)
	}
	return int(char) - 96
}
