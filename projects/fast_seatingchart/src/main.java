import java.io.*;
import java.nio.file.*;
import java.util.*;

public class main {
    private static final String INPUT_FILE = "./people.txt";
    private static final String OUTPUT_FILE = "./seating_chart.txt";

    public static void main(String[] args) throws IOException {
        long start = System.currentTimeMillis();
        code();
        System.out.println("Time: " + (System.currentTimeMillis() - start) + "ms");
    }

    private static int[] minRowsCols(int amount) {
        double sqrt = Math.sqrt(amount);
        int rows = (int) Math.ceil(sqrt);
        int cols = rows * (rows - 1) >= amount ? rows - 1 : rows;
        return new int[]{rows, cols};
    }

    private static void code() throws IOException {
        List<String> people = new ArrayList<>();
        Path path = Paths.get(INPUT_FILE);
        List<String> lines = Files.readAllLines(path);

        for (String line : lines) {
            people.add(line);
        }

        int[] rowsCols = minRowsCols(people.size());
        int rows = rowsCols[0];
        int cols = rowsCols[1];

        String[][] seatingChart = new String[rows][cols];
        for (int i = 0; i < people.size(); i++) {
            int row = i / cols;
            int col = i % cols;
            seatingChart[row][col] = people.get(i);
        }

        
        BufferedWriter output = new BufferedWriter(new FileWriter(OUTPUT_FILE));
        for (String[] row : seatingChart) {
            for (String person : row) {
                if (person != null) {
                    output.write(String.format("%-20s", person));
                }
            }
            output.newLine();
        }
        output.close();

        System.out.println("Seating chart written to " + OUTPUT_FILE);
    }
}