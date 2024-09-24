##remove prefus *.class
#Very Funny 
javac -g -parameters -s ./build @file_java.txt 
java -XX:+UseZGC  -XX:+PrintGCDetails -XX:+UseThreadPriorities -Xms10m -Xmx1024m -ea ./src/Main.java 

rm ./src/*.class 
rm ./src/cmd/*.class 