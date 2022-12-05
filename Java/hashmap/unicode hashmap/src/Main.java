import java.util.HashMap;

public class Main {
    public static void main(String[] args) {
        HashMap<String, String> emojis = new HashMap<>();
        emojis.put("heart", "\u2764");
        emojis.put("smile", "\uD83D\uDE00");
        emojis.put("sad", "\uD83D\uDE14");
        emojis.put("angry", "\uD83D\uDE20");
        emojis.put("laugh", "\uD83D\uDE02");
        System.out.println(emojis.get("heart"));
        System.out.println(emojis.get("smile"));
        System.out.println(emojis.get("sad"));
        System.out.println(emojis.get("angry"));
        System.out.println(emojis.get("laugh"));
    }
}