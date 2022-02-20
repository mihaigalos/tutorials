import rand
import time

const (
	element_count         = 2_000_000
	element_max_value     = 999
	minimum_element_count = 2
)

fn test_is_sorted_when_typical() {
	mut array := generate_data(element_count, element_max_value)
	println('Length of random array is $array.len')
	println('Is sorted: ${is_sorted<int>(array)}')
	println('--- Starting ---')

	start := time.now()
	quicksort<int>(mut array)
	end := time.now()

	println('--- Done ---')
	println('Is sorted: ${is_sorted<int>(array)}')
	println('Took: ' + ((end - start) / time.millisecond).str() + ' ms.')
}

fn generate_data(length int, element_max_valueimum int) []int {
	mut array := []int{cap: length}
	for _ in 0 .. length {
		array << rand.intn(element_max_valueimum)
	}

	return array
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

fn is_sorted<T>(array []T) bool {
	for i in 0 ..array.len - 1 {
		if array[i] > array[i + 1] {
			return false
		}
	}
	return true
}
