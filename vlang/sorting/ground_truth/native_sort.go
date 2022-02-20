package main

import (
	"fmt"
	"math/rand"
	"sort"
	"time"
)

func main() {

	data := generate_data(2000000)
	fmt.Println("--- Starting ---")
	start := time.Now()
	sort.Slice(data, func(i, j int) bool {
		return data[i] < data[j]
	})
	end := time.Now()
	fmt.Println("--- Done ---")
	fmt.Println("is sorted:", is_sorted(data))
	fmt.Print(end.Sub(start))

}

func generate_data(size int) []int {

	slice := make([]int, size, size)
	rand.Seed(time.Now().UnixNano())
	for i := 0; i < size; i++ {
		slice[i] = rand.Intn(999)
	}
	return slice
}

func is_sorted(a []int) bool {
	for i, _ := range a[0 : len(a)-1] {
		if a[i] > a[i+1] {
			return false
		}
	}
	return true
}
