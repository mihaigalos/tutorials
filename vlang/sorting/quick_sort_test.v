import rand
import time

const (
	element_count         = 2_000_000
	element_max_value     = 999
	minimum_element_count = 2
)

fn test_is_sorted_when_typical() {
	mut arr := generate_data(element_count, element_max_value)
	println('Length of random array is $arr.len')
	println('Is sorted: ${is_sorted<int>(arr)}')
	println('--- Starting ---')

	start := time.now()
	quicksort<int>(mut arr)
	end := time.now()

	println('--- Done ---')
	println('Is sorted: ${is_sorted<int>(arr)}')
	println('Took: ' + ((end - start) / time.millisecond).str() + ' ms.')
}

fn generate_data(length int, element_max_valueimum int) []int {
	mut arr := []int{cap: length}
	for _ in 0 .. length {
		arr << rand.intn(element_max_valueimum)
	}

	return arr
}

[direct_array_access]
fn quicksort<T>(mut a []T) {
	if a.len < minimum_element_count {
		return
	}

	mut left, mut right := 0, a.len - 1
	mut pivot := rand.intn(element_max_value) % a.len
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
