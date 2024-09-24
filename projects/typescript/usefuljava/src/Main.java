package src;

//https://ischool.uw.edu/diversity/events-programs/hack-social-good

public class Main  {
    static java.util.ArrayList<String> ScannerTokensToArray(java.util.Scanner arg) {
        java.util.ArrayList<String> ar = new java.util.ArrayList<String>();
        while(arg.hasNext()){
            ar.add(arg.next());
        }
        arg.close();
    
        return ar;
    }
    public static void main(String[] args){

    }
}