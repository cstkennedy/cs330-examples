package edu.odu.cs.cs330.examples;

import java.util.Arrays;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

import javax.vecmath.Vector3d;

/**
 * A coin flip driver.
 */
public class BuildMountain {

    /**
     * Number of microseconds in one second.
     */
    public static final double MICRO_SEC_TO_SEC = 1000000000.0;

    /**
     * Maximum allowed threads.
     */
    public static final int THREAD_LIMIT = 16;

    /**
     * Run Coin Flip Simulator.
     *
     * @param args[0] Number of threads to use [1,16]
     * @param args[1] Number of trials to run
     *
     */
    public static void main(String[] args)
    {
        int  numThreads = 1;

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

        long totalStartTime = System.nanoTime();

        long   parallelStartTime     = 0;
        long   parallelExecTime      = 0;
        double parallelExecTimeInSec = 0;

        if (numThreads == 1) {
            //singleThread();
        }
        else {
            try {
                /*parallelStartTime = System.nanoTime();

                //FlipTask[] completedTasks = multiThread(numTrials, numThreads);
                FlipTask[] completedTasks = multiThreadPool(numTrials, numThreads);

                parallelExecTime = System.nanoTime() - parallelStartTime;
                parallelExecTimeInSec = parallelExecTime / MICRO_SEC_TO_SEC;

                printOverallStatistics(completedTasks);

                System.out.println();
                System.out.println("Parallel Time: " + parallelExecTimeInSec);*/

                throw new InterruptedException();
            }
            catch (InterruptedException e) {
                System.err.println("Thread Error - Simulation incomplete\n");
            }
        }

        long totalExecTime = System.nanoTime() - totalStartTime;
        double totalExecTimeInSec = totalExecTime / MICRO_SEC_TO_SEC;

        System.out.println("Overall Time : " + totalExecTimeInSec);

        Triangle tri = new Triangle(new Vector3d(), new Vector3d(0, 1, 1), new Vector3d(0, 2, 0));
        System.out.println(tri);

        for(Triangle sTri : tri.split()) {
            System.out.println(sTri);
        }
    }
}
