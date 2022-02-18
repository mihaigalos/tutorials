fn f<T>(ch chan T, data T) { ch <- data }

fn main() {

    ch := chan string{}
	go f(ch, "ping")

    msg := <-ch
    println(msg)
}
