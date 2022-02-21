import rand
import time

const (
	element_count         = 2_000_000
	element_max_value     = 999
	minimum_element_count = 2
)

fn main() {
	mut array := generate_data(element_count, element_max_value)
	println('Length of random array is $array.len')
	println('Is sorted: ${is_sorted<int>(array)}')
	println('--- Starting ---')

	start := time.now()
	array.sort()
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

fn is_sorted<T>(array []T) bool {
	for i in 0 .. array.len - 1 {
		if array[i] > array[i + 1] {
			return false
		}
	}
	return true
}
