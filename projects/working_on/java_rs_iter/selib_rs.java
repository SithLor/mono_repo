class selib_rs {
    private static native String version();
    private static native byte[] rand_byte(int n);

    private static native long counterNew(selib_rs callback);
    private static native void counterIncrement(long counter_ptr);
    private static native void counterDestroy(long counter_ptr);

    private static native void asyncComputation(selib_rs callback);

    static {
        System.loadLibrary("selib_native_rs");
    }

    public static void main(String[] args) {

    }
}
