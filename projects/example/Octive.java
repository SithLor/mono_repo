class Octive {

    private native static void sort_array(int[] array);
    static {
        System.loadLibrary("octive");
    }

    public static void main(String[] args) {
        int[] huge_array_of_random_data = new int[1000000];
        // fill the array with random data
        for (int i = 0; i < huge_array_of_random_data.length; i++) {
            huge_array_of_random_data[i] = (int) (Math.random() * 1000000);
        }

        // get the current time in milliseconds and store it in a variable;
        long start = System.currentTimeMillis();
        // call the native method
        sort_array(huge_array_of_random_data);
        //get the end time in milliseconds and store it in a variable
        long end = System.currentTimeMillis();
        // print the difference between the end time and the start time
        System.out.println("native time taken: " + (end - start) + "ms");


    }
    static void not_sort_array(int[] array) {
        //quick sort
        Arrays.sort(array);
    }

}
