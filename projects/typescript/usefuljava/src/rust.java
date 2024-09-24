package src;

import src.Exception.StringLengthLessThanOne;

public class rust {
    
    public static String format(String Sformat, Object... AData) throws StringLengthLessThanOne {
        if (Sformat.length() == 0) {
            throw new StringLengthLessThanOne();
        }
        String replacedFormat = Sformat.replace("{}", "%s");
        return String.format(replacedFormat, AData);
    }
    public static void println(String dataString){
        System.out.println(dataString+"\n".toString());
        System.gc();
    }
}
