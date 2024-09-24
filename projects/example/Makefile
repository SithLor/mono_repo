java_run: lib
	javac Octive.java && java -Djava.library.path=octive/target/debug/ Octive

.PHONY: lib

javah:
	javac -h Octive.java

lib:
	cd octive && cargo build
