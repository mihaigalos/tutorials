fn publish<T>(channel chan T, data T) {
	channel <- data
}

fn test_generic_channel_works_when_typical_string() {
	expected := 'hello'
	channel := chan string{}

	go publish(channel, expected)
	actual := <-channel

	assert actual == expected
}

fn test_generic_channel_works_when_typical_int() {
	expected := 42
	channel := chan int{}

	go publish(channel, expected)
	actual := <-channel

	assert actual == expected
}
