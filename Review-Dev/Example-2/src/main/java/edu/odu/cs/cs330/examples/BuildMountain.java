package edu.odu.cs.cs330.examples;

import java.util.Arrays;
import java.util.Queue;
import java.util.List;
import java.util.ArrayList;
import java.util.LinkedList;

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

        Triangle tri = new Triangle(new Vector3d(),
                                    new Vector3d(0, 1, 1),
                                    new Vector3d(0, 2, 0));
        System.out.println(tri);

        for(Triangle sTri : tri.shiftCentroidAndSplit()) {
            System.out.println(sTri);
        }

        List<Triangle> listOfTris = recursivelySplit(tri);

        for(Triangle sTri : listOfTris) {
            System.out.printf("Smallest Side: %12.8f | Perimieter %12.8f%n", sTri.smallestSide(), sTri.perimeter());
            System.out.println(sTri);
        }

        System.out.printf("# Triangles %d%n", listOfTris.size());
        System.out.println();

        writeGNUPlotFmtOutput(listOfTris);
    }


    /**
     * Recursively split a triangle until one of four conditions is met:
     * <ol>
     *  <li> The length of any one side is less than or equal to <code>minLength</code>.
     *  <li> The height is less than or equal to <code>minHeight</code>
     *  <li> The area is less than or eaul to <code>minArea</code>.
     *  <li> The perimeter is less than or equal to <code>minPerimeter</code>
     * </ol>
     */
    static List<Triangle> recursivelySplit(Triangle startingTri)
    {
        List<Triangle> completedTris = new ArrayList<Triangle>();

        Queue<Triangle> procQ = new LinkedList();
        procQ.add(startingTri);

        while (procQ.peek() != null) {
            Triangle aTri = procQ.remove();

            for (Triangle childTri : aTri.shiftCentroidAndSplit()) {
                if (canBeSplitMore(childTri)) {
                    procQ.add(childTri);
                }
                else {
                    completedTris.add(childTri);
                }
            }
        }

        return completedTris;
    }

    /**
     * Determine if a triangle meets any of four conditions is met:
     * <ol>
     *  <li> The length of any one side is less than or equal to <code>minLength</code>.
     *  <li> The height is less than or equal to <code>minHeight</code>
     *  <li> The area is less than or eaul to <code>minArea</code>.
     *  <li> The perimeter is less than or equal to <code>minPerimeter</code>
     * </ol>
     *
     * @TODO finish
     */
    static boolean canBeSplitMore(Triangle aTri)
    {
        System.out.println("Smallest -> " + aTri.smallestSide());
        if (aTri.smallestSide() <= 1.0) {
            return false;
        }

        System.out.println("Perimeter -> " + aTri.perimeter());
        if (aTri.perimeter() <= 2) {
             return false;
        }

        // Placeholder
        return true;
    }

    /**
     * @TODO write documentation
     */
    static void writeGNUPlotFmtOutput(List<Triangle> triList)
    {
        for (Triangle tri : triList) {
            final Vector3d[] points = tri.getVertices();

            writeGNUPlotFmtPoint(points[0]);
            writeGNUPlotFmtPoint(points[1]);
            writeGNUPlotFmtPoint(points[2]);
            writeGNUPlotFmtPoint(points[0]);

            System.out.println();
        }
    }

    private static void  writeGNUPlotFmtPoint(final Vector3d pnt)
    {
        System.out.printf("%12.8f %12.8f %12.8f%n", pnt.x, pnt.y, pnt.z);
    }

}
