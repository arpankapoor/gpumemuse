show NVIDIA GPU memory usage by user:group

# compile

```console
$ cargo build --release
```

# example usage

```console
$ ./target/release/gpumemuse 
0: NVIDIA GeForce GTX 1080 Ti	 mem usage: 10.53/10.92GiB (96.43%)
	redacted1:redacted1: 62.05%
	redacted2:redacted2: 34.19%
	root:root: 0.15%
1: NVIDIA GeForce GTX 1080 Ti	 mem usage: 10.04/10.92GiB (91.97%)
	redacted1:redacted1: 91.94%
2: NVIDIA GeForce GTX 1080 Ti	 mem usage: 0.55/10.92GiB (5.07%)
	redacted3:redacted3: 5.04%
3: NVIDIA GeForce GTX 1080 Ti	 mem usage: 10.46/10.92GiB (95.84%)
	redacted1:redacted1: 95.80%
```
