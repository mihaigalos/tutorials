struct SquareIterator<T> {
	arr []T
mut:
	idx usize
}

fn (mut iter SquareIterator<T>) next<T>() ?T {
	if iter.idx >= iter.arr.len {
		return error('')
	}
	defer {
		iter.idx++
	}
	return iter.arr[iter.idx] * iter.arr[iter.idx]
}

fn test_square_int_works_when_typical() {
	expected := [1, 4, 9, 16, 25]
	input := [1, 2, 3, 4, 5]
	squares := SquareIterator<int>{
		arr: input
	}

	mut actual := []int{}
	for square in squares {
		actual << square
	}

	assert actual == expected
}

fn test_square_f32_works_when_typical() {
	expected := [2.25, 6.25, 12.25, 20.25, 30.25]
	input := [1.5, 2.5, 3.5, 4.5, 5.5]
	squares := SquareIterator<f64>{
		arr: input
	}

	mut actual := []f64{}
	for square in squares {
		actual << square
	}

	assert actual == expected
}