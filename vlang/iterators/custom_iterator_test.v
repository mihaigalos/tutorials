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

fn test_square_works_when_typical() {
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
