package edu.odu.cs.cs330.examples;

import java.util.Arrays;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

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
            singleThread();
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

        Triangle tri = new Triangle();
        System.out.println(tri);
    }

    /**
     * Single Thread Coin Flip.
     *
     * @param numTrials # flips to simulate
     */
    public static void singleThread()
    {
        /*FlipTask task = new FlipTask(numTrials);

        task.run();

        System.out.println(task);*/
    }

    /**
     * Multi-thread Coin Flip using a ThreadPool.
     *
     * @param numTrials # flips to simulate
     * @param numThreads number of threads to use
     *
     * @return Completed FlipTasks
     *
     * @throws InterruptedException if a thread is stopped prematurely
     */
    /*public static FlipTask[] multiThreadPool(long numTrials, int numThreads)
        throws InterruptedException
    {
        ExecutorService threadPool = Executors.newFixedThreadPool(numThreads);

        FlipTask[] tasks   = new FlipTask[numThreads];

        long trialsPerTask  = numTrials / numThreads;
        long trialsLastTask = trialsPerTask;

        // Add rounding error due to truncation
        trialsLastTask += numTrials - (trialsPerTask * numThreads);

        // Start threads n to n-2
        for (int i = 0; i < numThreads - 1; i++) {
            tasks[i] = new FlipTask(trialsPerTask);

            threadPool.execute(tasks[i]);
        }

        // Start thread n-1
        tasks[numThreads - 1] = new FlipTask(trialsLastTask);
        threadPool.execute(tasks[numThreads - 1]);

        threadPool.shutdown();
        threadPool.awaitTermination(60, TimeUnit.SECONDS);

        return tasks;
    }*/

    /**
     * Print overall multiThreaded statistics.
     *
     * @param tasks completed Flip Tasks
     */
/*    public static void printOverallStatistics(FlipTask[] tasks)
    {
        long totalHeads  = 0;
        long totalTails  = 0;
        long totalTrials = 0;

        for (int i = 0; i < tasks.length; i++) {
            System.out.format("Thread %2d -> %s", i, tasks[i]);

            totalTrials += tasks[i].numberTrials();
            totalHeads  += tasks[i].numberHeads();
            totalTails  += tasks[i].numberTails();
        }

        // Divider
        char[] divider = new char[72];
        Arrays.fill(divider, '-');
        System.out.println(new String(divider));

        System.out.format("Overall   -> # Heads: %6d (%6.4f) / # Tails %6d (%6.4f)%n",
                          totalHeads,
                          (1.0 * totalHeads / totalTrials),
                          totalTails,
                          (1.0 * totalTails / totalTrials));

        System.out.println("Total Trials " + totalTrials);
    }*/

}
