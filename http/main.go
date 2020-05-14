package main

import (
	"fmt"
	"net/http"
	"os"
)

func main() {
	response, err := http.Get("http://www.google.com")
	if err != nil {
		fmt.Println("Error: ", err)
		os.Exit(1)
	}

	bs := make([]byte, 9999)
	response.Body.Read(bs)

	fmt.Println(string(bs))
}
