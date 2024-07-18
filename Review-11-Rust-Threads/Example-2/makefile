java_run: lib
	javac Simulate.java && RUST_BACKTRACE=full
	java -Djava.library.path=./target/release Simulate $(ARGS)

.PHONY: lib

lib:
	cargo build --release
