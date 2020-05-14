package main

import (
	"fmt"
	"net/http"
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
		checkLink(link)
	}
}

func parallelExecution(links []string) {
	fmt.Println("Using Go Routines for parallel execution:")

	c := make(chan string)

	for _, link := range links {
		go checkLinkParallel(link, c)
	}

	for {
		go checkLinkParallel(<-c, c)
	}
}

func checkLink(link string) {
	_, e := http.Get(link)
	if e != nil {
		fmt.Println(link, "might be down!")
		return
	}
	fmt.Println(link, "is up.")
}

func checkLinkParallel(link string, c chan string) {
	_, e := http.Get(link)
	if e != nil {
		c <- link
		fmt.Println(link, "is down.")
		return
	}
	c <- link
	fmt.Println(link, "is up.")
}
