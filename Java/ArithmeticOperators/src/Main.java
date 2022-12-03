public class Main {
    public static void main(String[] args) {
        //Addition
        int addition = 12 + 12;
        double addition2 = 12.0 + 12.0;
        System.out.println(addition + " " + addition2);

        //Subtraction
        int subtraction = 12 - 11;
        double subtraction2 = 12.0 - 11.0;
        System.out.println(subtraction + " " + subtraction2);

        //Multiplication
        int multiplication = 5 * 5;
        double multiplication2 = 5.0 * 5.0;
        System.out.println(multiplication + " " + multiplication2);

        //Division
        int division = 25 / 5;
        double division2 = 25.0 / 5.0;
        System.out.println(division + " " + division2);

        //Modulus
        int mod = 25 % 4;
        System.out.println(mod);
        double mod2 = 66.0 % 11.0;
        System.out.println(mod2);

        //Compund Assignment Operators
        int plane = 0;
        plane++;
        // plane += 5;
        System.out.println(plane);

        int y = 245;
        int x = ++y;
        System.out.println(x);
    }
}