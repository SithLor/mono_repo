# qjvmll/Makefile

# Compile Java files
javac:  
	javac -d target/classes src/main/java/Server/*.java

# Run the Main class
run: javac	
	java -cp target/classes Server.Main

# Clean the build
clean:
	rm -rf target/classes

.PHONY: javac run clean