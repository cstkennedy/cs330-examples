package edu.odu.cs.cs330.examples;

import javax.vecmath.Vector3d;

/**
 * A triangle with three vertices in 3D space
 */
public class Triangle implements Cloneable {
    /**
     * 3 Triangle vertices in the form (x, y, x).
     */
    Vector3d[] vertices;

    /**
     * Construct a degenerate triangle
     * with all three vertices set to (0,0,0).
     */
    public Triangle()
    {
        vertices = new Vector3d[] {
            new Vector3d(0, 0, 0),
            new Vector3d(0, 0, 0),
            new Vector3d(0, 0, 0)
        };


    }

    /**
     *
     */
    @Override
    public Object clone()
    {
        return null;
    }
}