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
     * @return an x,y,z triple representing the centroid
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

        normalVector.cross(sideBA, sideBA);

        return normalVector;
    }

    /**
     * Split this Triangle at the centroid
     * and after shifting the centroid by the
     * normal
     */
    List<Triangle> split()
    {
        List<Triangle> resultTris = new ArrayList<Triangle>();

        Vector3d pntA = vertices[0];
        Vector3d pntB = vertices[1];
        Vector3d pntC = vertices[2];

        Vector3d pntCentroid = (Vector3d) this.centroid().clone();        

        Vector3d shiftedCentroid = new Vector3d();
        shiftedCentroid.scaleAdd(0.1, this.normal(), pntCentroid);

        resultTris.add(new Triangle(pntA, pntC, pntCentroid));
        resultTris.add(new Triangle(pntA, pntCentroid, pntB));
        resultTris.add(new Triangle(pntB, pntCentroid, pntC));

        return resultTris;
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