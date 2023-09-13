java: rust
	javac main.java && java -Djava.library.path=target/debug/ MyFirstRustClass

rust:
	cargo build
