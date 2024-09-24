java_run: lib
	javac selib_rs.java && java -Djava.library.path=selib_native_rs/target/debug/ selib_rs

.PHONY: lib

javah:
	javah selib_rs

lib:
	cd selib_native_rs && cargo build
