package Server;

public class Main {

    public static void main(String[] args) {
        String[] _args = new String[2];
        _args[0] = "AddJar";
        _args[1] = "/workspaces/octor/test/Main.jar";

        if (_args.length == 0) {
            System.out.println("Usage: java -cp <your-jar-file> Server.Main <command> [options]");
            System.out.println("Commands:");
            System.out.println("  AddJar <jarFilePath> - Add a JAR file to the thread manager");
            return;
        }

        String command = _args[0];

        switch (command) {
            case "AddJar":
                if (_args.length < 2) {
                    System.out.println("Usage: java -cp <your-jar-file> Server.Main AddJar <jarFilePath>");
                    return;
                }
                String jarFilePath = _args[1];
                Config config = new Config(2);
                ThreadManger threadManger = new ThreadManger(config);
                threadManger.AddJar(jarFilePath);
                break;
            case "Run":
                if (_args.length < 2) {
                    System.out.println("Usage: java -cp <your-jar-file> Server.Main Run <jarName>");
                    return;
                }
                String jarName = _args[1];
                
                break;
            default:
                System.out.println("Unknown command: " + command);
                System.out.println("Usage: java -cp <your-jar-file> Server.Main <command> [options]");
                System.out.println("Commands:");
                System.out.println("  AddJar <jarFilePath> - Add a JAR file to the thread manager");
                break;
        }
    }
}