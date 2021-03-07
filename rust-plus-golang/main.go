package main

/*
#cgo LDFLAGS: -L./lib -lhello
#include "./lib/hello.h"
*/
import "C"
import (
	"fmt"
	"time"
)

func main() {
	//C.hello(C.CString("John Smith"))
	start := time.Now()

	C.main_rust()
	rustBinaryTreesExecutionTime := time.Since(start)

	start = time.Now()
	binaryTree(21)
	goBinaryTreesExecutionTimes := time.Since(start)

	fmt.Println("Rust :: Binary Trees :: execution time: ", rustBinaryTreesExecutionTime)
	fmt.Println("Go   :: Binary Trees :: execution time: ", goBinaryTreesExecutionTimes)
}
