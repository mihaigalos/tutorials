package main

import "fmt"

func main() {

	colors := map[string]string{
		"red":   "0xFF0000",
		"green": "0x00FF00",
		"blue":  "0x0000FF",
	}

	delete(colors, "green")

	fmt.Println(colors)
}
