fn f(ch chan string) { ch <- "ping" }

fn main() {

    ch := chan string{}
	go f(ch)

    msg := <-ch
    println(msg)
}
