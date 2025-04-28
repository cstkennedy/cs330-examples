import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.stream.Stream;

/*
/// Note what happens if we do not check the
/// index entered by the user
///
/// De-Morgan's Law
/// !(index >= 3 && index <= 20)
/// !(index >= 3) || !(index <= 20)
/// (index < 3 || index > 20)
///
/// Rust
/// (3..=20).contains(...)
fn __validate_args(index: u8) -> bool {
    !(3..=20).contains(&index)
}

/// Generate the Fibonacci Sequence to the n-th number.
/// 1 1 2 3 5 8 13 21 34...
/// <p>
/// The user must enter a number no smaller than 3 and
/// no greater than 20
*/

public class Fibonacci
{
    private static class Pair<T>
    {
        public T left;
        public T right;

        public Pair(T left, T right)
        {
            this.left = left;
            this.right = right;
        }
    }

    public static void main(String... args)
        throws IOException
    {
        //--------------------------------------------------------------------------
        // Prompt for sequence length and validate
        //--------------------------------------------------------------------------
        System.out.print("Generate how many numbers? ");
        String line = new BufferedReader(
            new InputStreamReader(System.in)
        ).readLine();

        final int index;
        try {
            index = Integer.parseInt(line);
        }
        catch (NumberFormatException err) {
            throw new IllegalArgumentException("ERROR: Not a legal integer.", err);
        }

        System.out.println();


        /*
        if __validate_args(index) {
            eyre::bail!("{index:3} is not between 3 and 20");
        }
        */

        //--------------------------------------------------------------------------
        // Compute and output the Fibonaccci Sequence to the index-th term
        //--------------------------------------------------------------------------
        long[] fibonacciSeq = Stream.iterate(
            new Pair<Long>(0L, 1L),
            (pair) -> {
                long f_next = pair.left + pair.right;

                return new Pair<Long>(pair.right, f_next);
            }
        )
        .mapToLong(pair -> pair.right)
        .limit(index)
        .toArray();

        for (int idx = 0; idx < fibonacciSeq.length; ++idx) {
            System.out.printf("%2d: %10d%n", idx + 1, fibonacciSeq[idx]);
        }
    }
}
