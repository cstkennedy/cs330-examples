
public class Demo
{
    public static void main(String... args)
    {
        for (int i = 0; i < args.length; ++i) {
            System.out.printf("%3d: %s%n", i, args[i]);
        }
    }
}

