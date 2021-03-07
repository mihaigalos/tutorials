# Context

Side-by side Go native implementation of binary trees vs Rust native delegation (factor 7 faster).

The below is an aggregation of multiple sources, mostly [rust-plus-golang](https://github.com/mediremi/rust-plus-golang), [shootout-binary-trees](https://github.com/stepancheg/rust-ide-rust/blob/master/src/test/bench/shootout-binarytrees.rs).

# Results
```bash
$ ./binary_tree
Running Rust :: binaryTree (called from Go).
stretch tree of depth 22	 check: 8388607
2097152	 trees of depth 4	 check: 65011712
524288	 trees of depth 6	 check: 66584576
131072	 trees of depth 8	 check: 66977792
32768	 trees of depth 10	 check: 67076096
8192	 trees of depth 12	 check: 67100672
2048	 trees of depth 14	 check: 67106816
512	 trees of depth 16	 check: 67108352
128	 trees of depth 18	 check: 67108736
32	 trees of depth 20	 check: 67108832
long lived tree of depth 21	 check: 4194303

Running Go :: binaryTree (native Go implementation).
stretch tree of depth 22	 check: 8388607
2097152	 trees of depth 4	 check: 65011712
524288	 trees of depth 6	 check: 66584576
131072	 trees of depth 8	 check: 66977792
32768	 trees of depth 10	 check: 67076096
8192	 trees of depth 12	 check: 67100672
2048	 trees of depth 14	 check: 67106816
512	 trees of depth 16	 check: 67108352
128	 trees of depth 18	 check: 67108736
32	 trees of depth 20	 check: 67108832
long lived tree of depth 21	 check: 4194303

Rust :: Binary Trees :: execution time:  1.547774462s
Go   :: Binary Trees :: execution time:  12.291931217s

$ lscpu
Architecture:        x86_64
CPU op-mode(s):      32-bit, 64-bit
Byte Order:          Little Endian
CPU(s):              4
On-line CPU(s) list: 0-3
Thread(s) per core:  2
Core(s) per socket:  2
Socket(s):           1
NUMA node(s):        1
Vendor ID:           GenuineIntel
CPU family:          6
Model:               60
Model name:          Intel(R) Core(TM) i5-4570T CPU @ 2.90GHz
Stepping:            3
CPU MHz:             1278.149
CPU max MHz:         3600,0000
CPU min MHz:         800,0000
BogoMIPS:            5787.11
Virtualization:      VT-x
L1d cache:           32K
L1i cache:           32K
L2 cache:            256K
L3 cache:            4096K
NUMA node0 CPU(s):   0-3
Flags:               fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf pni pclmulqdq dtes64 monitor ds_cpl vmx smx est tm2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm cpuid_fault epb invpcid_single pti ssbd ibrs ibpb stibp tpr_shadow vnmi flexpriority ept vpid fsgsbase tsc_adjust bmi1 avx2 smep bmi2 erms invpcid xsaveopt dtherm ida arat pln pts md_clear flush_l1d
```

## Rust + Golang
---

This repository shows how, by combining
[`cgo`](https://blog.golang.org/c-go-cgo) and
[Rust's FFI capabilities](https://doc.rust-lang.org/book/ffi.html), we can call
Rust code from Golang.

Run `make build` and then `./main` to see `Rust` + `Golang` in action. You
should see `Hello John Smith!` printed to `stdout`.

### You can do this for your own project
Begin by creating a `lib` directory, where you will keep your Rust libraries.
[Andrew Oppenlander's article on creating a Rust dynamic library is a great introduction](http://oppenlander.me/articles/rust-ffi).

Then, you need to create a C header file for your library. Just copy the `libc`
types that you used.

All that is left to do is to add some `cgo`-specific comments to your Golang
code. These comments tell `cgo` where to find the library and its headers.

```go
/*
#cgo LDFLAGS: -L./lib -lhello
#include "./lib/hello.h"
*/
import "C"
```

> There should not be a newline between `*/` and `import "C"`.

A simple `make build` (use the [Makefile](Makefile) in this repository) will
result in a binary that loads your dynamic library.
