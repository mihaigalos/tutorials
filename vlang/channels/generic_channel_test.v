fn f<T>(ch chan T, data T) { ch <- data }

fn test_generic_channel_works_when_typical_string() {
    expected := "hello"
    ch := chan string{}

	go f(ch, expected)
    actual := <-ch

    assert actual == expected
}