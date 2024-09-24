class Json {

    //fast json libary for java
    private static final Js factory = new JsonFactory();

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("mylib");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        String output = Rust.hello("josh");
        System.out.println(output);
    }
}