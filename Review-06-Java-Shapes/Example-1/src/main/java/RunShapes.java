//Programmer : Thomas Kennedy

import utilities.Utilities;
import shapes.*;

public class RunShapes {
    /**
     * This is the Program Title.
     */
    private static final String[] PROGRAM_HEADING = {
        "Objects & Inheritance: 2-D Shapes",
        "Thomas J. Kennedy"
    };

    /**
     * The default heading width.
     */
    private static final int H_WIDTH = 38;

    /**
     * This is the main function.
     */
    public static void main(String[] args)
    {
        // --Erroneous C++ Variable Declarations--
        // --Valid Java Variable Declarations--
        Shape               shape  = null; // Declare an "instance" of Shape
        Shape[]             shapes = null; // Declare an Array of Shapes

        Triangle            tri    = null;
        RightTriangle       rhtTri = null;
        EquilateralTriangle eqlTri = null;

        // Print main program heading
        System.out.println(
            Utilities.projectHeading(PROGRAM_HEADING, Utilities.W_WIDTH)
        );

        //----------------------------------------------------------------------
        // Create one RightTriangle
        //----------------------------------------------------------------------
        rhtTri  = new RightTriangle(1, 2);

        tri     = rhtTri; // Point tri to rhtTri
        shape   = rhtTri; // Point shape to rhtTri

        /*
        // Is this a valid assignment?
        eqlTri = rhtTri;
        */

        System.out.println(
            Utilities.seperatedHeading("Display a Right Triangle (rhtTri)", H_WIDTH, '-')
        );
        System.out.println(rhtTri);

        System.out.println(
            Utilities.seperatedHeading("Display a Right Triangle (tri)", H_WIDTH, '-')
        );
        System.out.println(tri);

        System.out.println(
            Utilities.seperatedHeading("Display a Right Triangle (shape)", H_WIDTH, '-')
        );
        System.out.println(shape);

        //----------------------------------------------------------------------
        // Divide Output - Separate Right Triangle from Eql. Triangle
        //----------------------------------------------------------------------
        System.out.println(
            Utilities.horizontalLine('~', Utilities.W_WIDTH)
        );
        System.out.println();

        rhtTri = null;

        // Create one Equilateral Triangle
        eqlTri = new EquilateralTriangle(8);

        tri    = eqlTri; // Point tri to rhtTri
        shape  = eqlTri; // Point shape to rhtTri

        System.out.println(
            Utilities.seperatedHeading("Display an Eql. Triangle (eqlTri)", H_WIDTH, '-')
        );
        System.out.println(eqlTri);

        System.out.println(
            Utilities.seperatedHeading("Display an Eql. Triangle (tri)", H_WIDTH, '-')
        );
        System.out.println(tri);

        System.out.println(
            Utilities.seperatedHeading("Display an Eql. Triangle (shape)", H_WIDTH, '-')
        );
        System.out.println(shape);
    }
}
