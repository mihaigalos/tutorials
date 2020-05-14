package main

import "fmt"

func main() {

	colors := map[string]string{
		"red":   "0xFF0000",
		"green": "0x00FF00",
		"blue":  "0x0000FF",
	}

	printMap(colors)

	fmt.Println(colors)
}

func printMap(c map[string]string) {
	for k, v := range c {
		fmt.Printf("Key: %v ", k)
		fmt.Println("Value: ", v)
	}
}
