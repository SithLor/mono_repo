package Server;

import java.util.Scanner;

public class Manifest {
    public String version;
    public String main_class_name;
    public String created_by;

    public Manifest(Scanner scanner) {
        version = scanner.nextLine().split(": ")[1];
        main_class_name = scanner.nextLine().split(": ")[1];
        created_by = scanner.nextLine().split(": ")[1];
    }
}