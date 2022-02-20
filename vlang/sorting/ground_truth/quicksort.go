package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {

	slice := generate_data(2_000_000)
	fmt.Println("--- Starting ---")
	start := time.Now()
	a := quicksort(slice)
	end := time.Now()
	fmt.Println("--- Done ---")
	fmt.Println("is sorted:", is_sorted(a))
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

func quicksort(a []int) []int {
	if len(a) < 2 {
		return a
	}

	left, right := 0, len(a)-1
	pivot := rand.Int() % len(a)
	a[pivot], a[right] = a[right], a[pivot]

	for i, _ := range a {
		if a[i] < a[right] {
			a[left], a[i] = a[i], a[left]
			left++
		}
	}

	a[left], a[right] = a[right], a[left]

	quicksort(a[:left])
	quicksort(a[left+1:])

	return a
}

func is_sorted(a []int) bool {
	for i, _ := range a[0 : len(a)-1] {
		if a[i] > a[i+1] {
			return false
		}
	}
	return true
}
