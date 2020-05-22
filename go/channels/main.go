package main

import (
	"fmt"
	"net/http"
	"time"
)

func main() {
	links := []string{
		"http://amazon.com",
		"http://facebook.com",
		"http://golang.org",
		"http://google.com",
		"http://stackoverflow.com",
		"http://wikipedia.com",
	}

	serialExecution(links)
	fmt.Println("------------")
	parallelExecution(links)

}

func serialExecution(links []string) {
	fmt.Println("Using serial execution:")
	for _, link := range links {
		checkLinkSerial(link)
	}
}

func parallelExecution(links []string) {
	fmt.Println("Using Go Routines for parallel execution:")

	c := make(chan string)

	for _, link := range links {
		go checkLinkParallel(link, c)
	}

	for l := range c {
		go func(l string) {
			time.Sleep(5 * time.Second)
			checkLinkParallel(l, c)
		}(l)
	}
}

func checkLinkSerial(link string) {
	_, e := http.Get(link)
	if e != nil {
		fmt.Println(link, "might be down!")
		return
	}
	fmt.Println(link, "is up.")
}

func checkLinkParallel(link string, l chan string) {
	_, e := http.Get(link)
	if e != nil {
		l <- link
		fmt.Println(link, "is down.")
		return
	}
	l <- link
	fmt.Println(link, "is up.")
}
