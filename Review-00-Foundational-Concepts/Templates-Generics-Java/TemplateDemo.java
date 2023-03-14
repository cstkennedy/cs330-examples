
public class TemplateDemo
{

	/**
	 * Print a value multiple times.
	 *
	 * @param <T> type to print multiple times
	 *
	 * @param value value to print
	 * @param count number of times to print
	 */
	public static <T> void printMultipleTimes(T value, final int count)
	{
		for (int i = 0; i < count - 1; i++) {
			System.out.printf("%s ", value);
		}
        System.out.printf("%s%n", value);
	}


	public static void main(String... argv)
	{
		printMultipleTimes(7, 3);
		System.out.println();
		printMultipleTimes(5.5, 2);
		System.out.println();
		printMultipleTimes(5.5f, 2);
		System.out.println();
		printMultipleTimes("Hello", 4);
		System.out.println();
		printMultipleTimes('?', 20);
		System.out.println();
	}
}
