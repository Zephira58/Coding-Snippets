public class Main {
    public static void main(String[] args) {
        byte byteyboi = 7;
        System.out.println(byteyboi);

        short shortyboi = 32000;
        System.out.println(shortyboi);

        int  intyboi = 2000000000;
        System.out.println(intyboi);

        long longyboi = 9000000000000000000L;
        System.out.println(longyboi+"\n");

        //Compute the distance light travels using a long.
        long lightSpeed = 186000;
        long days = 1000;
        long seconds = days * 24 * 60 * 60;
        long distance = lightSpeed * seconds;
        System.out.println("In " + days + distance + " days, the light will have traveled " + distance + " miles.\n");

        float myBankAccount = 12.99f;
        System.out.println(myBankAccount);

        double ricepower90istrash = 0.7843902;
        System.out.println(ricepower90istrash);

        //find the area of a circle using a double
        double pi, r, a;
        r = 8.7;
        pi = 3.141592653589793238462643383279502884197169399375105820974944592307816406286;
        a = pi * r * r;
        System.out.println("\nThe area of a circle with radius " + r + " is " + a + ".\n");

        //https://unicode-table.com/en/2661/
        char letter = '\u2661';
        System.out.println(letter);

        // UTF-16BE dec
        char heart = 9825;
        System.out.println(heart);

        /*
        char unicodeChar = 1;
        for(int i = 0; i < 50000;i++) {
            System.out.println(unicodeChar);
            unicodeChar++;
        }
        */

        boolean imCool = true;
        boolean paxtonIscool = false;
        System.out.println(imCool);
        System.out.println(paxtonIscool);

    }
}