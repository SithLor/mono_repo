import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        int[] test = new int[1000];
        StringBuilder testString = new StringBuilder("Hello, World! from Test.jar");

        // Number of threads for array filling
        int numFillThreads = 1000;
        int chunkSize = test.length / numFillThreads;
        List<Thread> fillThreads = new ArrayList<>();

        // Create and start filling threads
        for (int i = 0; i < numFillThreads; i++) {
            int start = i * chunkSize;
            int end = (i == numFillThreads - 1) ? test.length : start + chunkSize;
            Thread fillThread = new Thread(new ArrayFiller(test, start, end));
            fillThreads.add(fillThread);
            fillThread.start();
        }

        // Wait for all filling threads to finish
        for (Thread fillThread : fillThreads) {
            try {
                fillThread.join();
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }

        System.out.println("Array filling completed.");

        // Number of threads for concatenation
        int numConcatThreads = 100;
        chunkSize = test.length / numConcatThreads;
        List<Thread> concatThreads = new ArrayList<>();

        // Create and start concatenation threads
        for (int i = 0; i < numConcatThreads; i++) {
            int start = i * chunkSize;
            int end = (i == numConcatThreads - 1) ? test.length : start + chunkSize;
            Thread concatThread = new Thread(new ArrayConcatenator(test, testString, start, end));
            concatThreads.add(concatThread);
            concatThread.start();
        }

        // Wait for all concatenation threads to finish
        for (Thread concatThread : concatThreads) {
            try {
                concatThread.join();
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }

        System.out.println("Concatenation completed.");
        System.out.println("Result: " + testString.toString());
    }
}

class ArrayFiller implements Runnable {
    private int[] array;
    private int start;
    private int end;

    public ArrayFiller(int[] array, int start, int end) {
        this.array = array;
        this.start = start;
        this.end = end;
    }

    @Override
    public void run() {
        for (int i = start; i < end; i++) {
            array[i] = (int) (Math.random() * 1000000);
            System.out.println("Thread " + Thread.currentThread().getName() + " is filling index " + i);
        }
    }
}

class ArrayConcatenator implements Runnable {
    private int[] array;
    private StringBuilder stringBuilder;
    private int start;
    private int end;

    public ArrayConcatenator(int[] array, StringBuilder stringBuilder, int start, int end) {
        this.array = array;
        this.stringBuilder = stringBuilder;
        this.start = start;
        this.end = end;
    }

    @Override
    public void run() {
        for (int i = start; i < end; i++) {
            synchronized (stringBuilder) {
                stringBuilder.append(array[i]);
            }
            System.out.println("run concat");
        }
    }
}