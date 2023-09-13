class MyFirstRustClass {
    // This declares that the static `hello` method will be provided
    // a native library.
    private static native void example();

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("rust_java");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        System.out.println("Hello World");
        MyFirstRustClass.example();
    }
}