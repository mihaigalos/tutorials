import rand
import time

const (
	len = 2000000 // how many random numbers to generate
	max = 999 // max of the generated numbers
)

fn main() {
	mut arr := generate_data(len, max)
	println('length of random array is $arr.len')
	println('is sorted: ${is_sorted<int>(arr)}')
	println('--- Starting ---')

	start := time.now()
	quicksort<int>(mut arr)
	end := time.now()

	println('--- Done ---')
	println('is sorted: ${is_sorted<int>(arr)}')

	print(end - start)
	println('s')
}

fn generate_data(length int, maximum int) []int {
	mut arr := []int{cap: length}
	for _ in 0 .. length {
		arr << rand.intn(maximum)
	}

	return arr
}

[direct_array_access]
fn quicksort<T>(mut a []T) {
	if a.len < 2 {
		return
	}

	mut left, mut right := 0, a.len - 1
	mut pivot := rand.intn(max) % a.len
	a[pivot], a[right] = a[right], a[pivot]

	for i, _ in a {
		if a[i] < a[right] {
			a[left], a[i] = a[i], a[left]
			left++
		}
	}

	a[left], a[right] = a[right], a[left]

	quicksort(mut a[..left])
	quicksort(mut a[left + 1..])
}

fn is_sorted<T>(arr []T) bool {
	for i in 0 .. arr.len - 1 {
		if arr[i] > arr[i + 1] {
			return false
		}
	}
	return true
}
