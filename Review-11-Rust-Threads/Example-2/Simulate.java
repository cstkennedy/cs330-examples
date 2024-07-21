
public class Simulate
{
    static {
        System.loadLibrary("coin_flip_java");
    }

    public static native void doFlips(int numThreads, long numTrials);

    /**
     * Number of microseconds in one second.
     */
    public static final double MICRO_SEC_TO_SEC = 1000000000.0;

    /**
     * Default number of trials to run.
     */
    public static final long DEFAULT_NUM_TRIALS = 10000;

    /**
     * Maximum allowed threads.
     */
    public static final int THREAD_LIMIT = 32;

    /**
     * Run Coin Flip Simulator.
     *
     * @param args[0] Number of threads to use [1,32]
     * @param args[1] Number of trials to run
     *
     */
    public static void main(String... args)
    {
        int  numThreads = 1;
        long numTrials  = DEFAULT_NUM_TRIALS;

        // Thread argument validation
        try {
            numThreads = Integer.parseInt(args[0]);
        }
        catch (NumberFormatException e) {
            numThreads = 1;
        }
        catch (IndexOutOfBoundsException e) {
            numThreads = 1;
        }

        if (numThreads > THREAD_LIMIT) {
            numThreads = THREAD_LIMIT;
        }

        // # Trial argument validation
        try {
            numTrials = Long.parseLong(args[1]);
        }
        catch (NumberFormatException e) {
            numTrials = DEFAULT_NUM_TRIALS;
        }
        catch (IndexOutOfBoundsException e) {
            numTrials = DEFAULT_NUM_TRIALS;
        }

        Simulate.doFlips(numThreads, numTrials);
    }
}
