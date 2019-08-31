package edu.odu.cs.cs330.examples;

import java.util.List;
import java.util.ArrayList;

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
     * Construct a triangle
     * with three specified vertices.
     */
    public Triangle(Vector3d ptA, Vector3d ptB, Vector3d ptC)
    {
        vertices = new Vector3d[] {
            (Vector3d) ptA.clone(),
            (Vector3d) ptB.clone(),
            (Vector3d) ptC.clone()
        };
    }

    /**
     * Return the three vertices v0, v1, v2 in order.
     */
    final Vector3d[] getVertices()
    {
        return vertices;
    }

    /**
     * Compute the centroid
     *
     * @return an x,y,z triple representing the centroid
     */
    Vector3d centroid()
    {
        Vector3d centroid = new Vector3d(0, 0, 0);

        for (Vector3d vertex : vertices) {
            centroid.x += vertex.x;
            centroid.y += vertex.y;
            centroid.z += vertex.z;
        }

        centroid.x /= 3.0;
        centroid.y /= 3.0;
        centroid.z /= 3.0;

        return centroid;
    }

    /**
     * Compute the normal vector
     *
     * @return an x,y,z triple representing the normal
     */
    Vector3d normal()
    {
        Vector3d pntA = vertices[0];
        Vector3d pntB = vertices[1];
        Vector3d pntC = vertices[2];

        Vector3d sideBA = new Vector3d();
        Vector3d sideCA = new Vector3d();

        sideBA.sub(pntB, pntA);
        sideCA.sub(pntB, pntC);

        Vector3d normalVector = new Vector3d();

        normalVector.cross(sideBA, sideCA);

        return normalVector;
    }

    /**
     * Split this Triangle at the centroid
     * and after shifting the centroid by the
     * normal.
     *
     * @TODO Replace hardcoded 0.1 with constant or arg.
     */
    List<Triangle> shiftCentroidAndSplit()
    {
        List<Triangle> resultTris = new ArrayList<Triangle>();

        Vector3d pntA = vertices[0];
        Vector3d pntB = vertices[1];
        Vector3d pntC = vertices[2];

        Vector3d pntCentroid = (Vector3d) this.centroid().clone();

        // Vector3d shiftedCentroid = new Vector3d();
        // shiftedCentroid.scaleAdd(0.1, this.normal(), pntCentroid);

        resultTris.add(new Triangle(pntA, pntC, pntCentroid));
        resultTris.add(new Triangle(pntA, pntCentroid, pntB));
        resultTris.add(new Triangle(pntB, pntCentroid, pntC));

        return resultTris;
    }

    private double computeSide(Vector3d v1, Vector3d v2)
    {
        double x = v2.x - v1.x;
        double y = v2.y - v1.y;
        double z = v2.z - v1.z;

        return Math.sqrt(Math.pow(x, 2.0)
                       + Math.pow(y, 2.0)
                       + Math.pow(z, 2.0));
    }

    public double[] computeSides()
    {
        return new double[] {
            computeSide(vertices[0], vertices[1]),
            computeSide(vertices[1], vertices[2]),
            computeSide(vertices[2], vertices[0])
        };
    }

    public double smallestSide()
    {
        double[] sideLengths = this.computeSides();

        return Math.min(Math.min(sideLengths[0],
                                 sideLengths[1]),
                        sideLengths[2]);
    }

    /**
     * Compute perimeter using vector arithmetic.
     *
     * @TODO implement
     */
    double perimeter()
    {
        double[] sideLengths = this.computeSides();

        return sideLengths[0]
             + sideLengths[1]
             + sideLengths[2];
    }

    /**
     * Use the semi-perimeter method to compute area.
     *
     * @TODO implement
     */
    double area()
    {
        return -1;
    }

    /**
     * Create a deep copy
     */
    @Override
    public Object clone()
    {
        return null;
    }

    /**
     *
     */
    @Override
    public String toString()
    {
        return vertices[0] + " " + vertices[1] + " " + vertices[2];
    }
}
