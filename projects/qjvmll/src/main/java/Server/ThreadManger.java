package Server;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Path;
import java.io.FileNotFoundException; // Import this class to handle errors
import java.nio.file.StandardCopyOption;
import java.util.Enumeration;
import java.util.List;
import java.util.Scanner;
import java.util.jar.JarEntry;
import java.util.jar.JarFile;
import java.util.ArrayList;


public class ThreadManger {
    public String[] JarsPath;
    public String[] JarsName;
    public int JarsCount = 0;

    public ThreadManger(Config config) {
        JarsPath = new String[config.thread_count];
        JarsName = new String[config.thread_count];
    }

    public void unpackJar(String jarFilePath, File destDir) throws IOException {
        try (JarFile jarFile = new JarFile(jarFilePath)) {
            Enumeration<JarEntry> entries = jarFile.entries();

            while (entries.hasMoreElements()) {
                JarEntry entry = entries.nextElement();
                File entryDestination = new File(destDir, entry.getName());

                if (entry.isDirectory()) {
                    entryDestination.mkdirs();
                } else {
                    entryDestination.getParentFile().mkdirs();
                    try (InputStream in = jarFile.getInputStream(entry);
                            FileOutputStream out = new FileOutputStream(entryDestination)) {
                        byte[] buffer = new byte[1024];
                        int bytesRead;
                        while ((bytesRead = in.read(buffer)) != -1) {
                            out.write(buffer, 0, bytesRead);
                        }
                    }
                }
            }
        }
    }

    public void AddJar(String JarPath) {
        String JarName = JarPath.substring(JarPath.lastIndexOf("/") + 1);
        Path tempDir;

        // add the name of the jar
        JarsName[JarsCount] = JarName;

        try {
            // Create a temporary directory
            tempDir = Files.createTempDirectory("temp_" + JarName);

            // Unpack the JAR file into the temporary directory
            unpackJar(JarPath, tempDir.toFile());

            JarsPath[JarsCount] = tempDir.toString();

            // Load the JAR file classes
            File[] classes_ = tempDir.toFile().listFiles();

            System.out.print("JAR UNPACKED AT:" + tempDir.toString());
            for (File file : classes_) {
                if(file.isDirectory()){
                    String dirName = file.getName();
                    String file_in_dir = file.listFiles()[0].getName();
                    System.out.print("\n /" + dirName + "/" + file_in_dir);
                } else {
                    System.out.print("\n /" + file.getName());
                }
            }

            JarsCount++;
            System.out.print("\n");
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public void StartJar(int index) {
        // read from JarsPath[index]/META-INF/MANIFEST.MF
        File manifest_file = new File(JarsPath[index - 1] + "/META-INF/MANIFEST.MF");
        // read the file as text
        Manifest t_manifest = null;
        try {
            Scanner scanner = new Scanner(manifest_file);

            t_manifest = new Manifest(scanner);
            scanner.close();
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
        Thread t = new Thread() {
            public void run() {
                try {
                    // Load the main class
                    Class<?> main_class = Class.forName("Main.class");
        
                    // Create an instance of the main class
                    Object main_instance = main_class.getDeclaredConstructor().newInstance();
        
                    // Get the main method
                    Method[] main_method = main_class.getMethods()
        
                    // Invoke the main method
                    String[] args = {}; // You can pass command-line arguments here if needed
                    main_method.invoke(null, (Object) args);
                } catch (ClassNotFoundException e) {
                    e.printStackTrace();
                } catch (InstantiationException e) {
                    e.printStackTrace();
                } catch (IllegalAccessException e) {
                    e.printStackTrace();
                } catch (NoSuchMethodException e) {
                    e.printStackTrace();
                } catch (java.lang.reflect.InvocationTargetException e) {
                    e.printStackTrace();
                }
            }
        };
        t.start();
    }
}