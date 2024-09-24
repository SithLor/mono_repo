javac ./Main.java

java -XX:+UseG1GC -XX:+OptimizeStringConcat -Xcomp Main.java
