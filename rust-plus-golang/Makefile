ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

build:
	cd lib/hello && cargo build --release
	cp lib/hello/target/release/libhello.so lib/
	go build -ldflags="-r $(ROOT_DIR)lib" main.go binary_tree.go

run: build
	./main
