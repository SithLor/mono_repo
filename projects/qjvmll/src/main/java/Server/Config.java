package Server;

public class Config {
    public int thread_count;
    public Config(int thread_counts) {
        if (thread_counts < 0) {
            System.out.println("ERROR: Thread count must be greater than 0");
            System.exit(1);
        }
        thread_count = thread_counts;
    }
}