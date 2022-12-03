public class Main {
    public static void main(String[] args) {
        // [datatype] [name]
        int daysInWeek[] = {111, 222, 333 ,444, 555, 666, 777};
        System.out.println(daysInWeek[0]);
        System.out.println(daysInWeek[1]);
        System.out.println(daysInWeek[2]);
        System.out.println(daysInWeek[3]);
        System.out.println(daysInWeek[4]);
        System.out.println(daysInWeek[5]);
        System.out.println(daysInWeek[6]);

        double[] prices = {14.2, 13.4, 14.5, 52.3};
        double sum = prices[0] + prices[1] + prices[2] + prices[3];
        double averageSum = sum / 4;
        System.out.println(averageSum);
    }
}