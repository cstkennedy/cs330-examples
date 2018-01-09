//Programmer : Thomas Kennedy

import utilities.Utilities;
import shapes.*;

public class RunShapes {
    static final String[] PROGRAM_HEADING = {
        "Objects & Inheritance: 2-D Shapes",
        "Thomas Kennedy"
    };  ///< Program Title 

    /**
     *
     */
    public static void main(String args[])
    {
        // --Erroneous C++ Variable Declarations--
        // --Valid Java Variable Declarations--
        Shape               shape  = null; // Declare an "instance" of Shape
        Shape[]             shapes = null; // Declare an Array of Shapes
        
        // ShapeFactory Discussion
        int size = 0;
        
        // Print main program heading
        System.out.println(
            Utilities.projectHeading(PROGRAM_HEADING, Utilities.W_WIDTH)
       );

       /*
        * What happens when the number of shapes is non-trivial?
        *
        * Suppose we were to expand our Shape hierarchy to include
        * the following shapes:
        *   - Isosceles Triangle 
        *   - Circle
        *   - Ellipse
        *   - Rectangle
        *   - Square
        *   - Rhombus
        *   - Parallelogram
        *   - Kite
        *   - Generalized Polygon
        *
        * How would we manage the addition of new Shapes?
        *
        * A common approach is to make use of the Factory Model.
        * This Model exists in a number of languages--e.g.:
        *   - C++
        *   - Java
        *   - Python
        *   - PHP
        *   - C#
        * 
        * A class that contains static members is created.
        * As new classes are created, the Factory Class is
        * updated.
        *
        * In this example, our factory class is called ShapeFactory
        * The ShapeFactory could be designed as a singleton class.
        * Our ShapeFactory is simply a tracker--i.e., records are static
        * and will be updated manually at compile time.
        *
        * The Singleton Class implementation may be discussed at a
        * later date
        */

        // Examine the ShapeFactory
        System.out.println(Utilities.heading("Available Shapes", 38, '*'));

        // List the available shapes
        System.out.print(ShapeFactory.listKnown());
        System.out.println(Utilities.horizontalLine('-', 38));
        System.out.printf("%2d shapes available.\n", ShapeFactory.numberKnown());    

        System.out.println();

        // Create 5 "Random" Shapes
        size   = 6; // original size of the array
        shapes = new Shape[ size ];

        shapes[0] = ShapeFactory.createShape("Triangle");
        shapes[1] = ShapeFactory.createShape("Right Triangle");
        shapes[2] = ShapeFactory.createShape("Equilateral Triangle");
        shapes[3] = ShapeFactory.createShape("Square");
        shapes[4] = ShapeFactory.createShape("Circle");
        shapes[5] = ShapeFactory.createShape("1337 Haxor");

        shapes = pruneNullPtr(shapes);        
        
        System.out.println(Utilities.heading("Shapes That Exist", 38, '*'));
        System.out.printf("%-24s: %4d\n", "Original Size", size);     
        System.out.printf("%-24s: %4d\n", "Invalid Shapes", (size - shapes.length));
        System.out.printf("%-24s: %4d\n", "New Size", shapes.length);
        
        System.out.println();

        // Print all the shapes
        System.out.println(Utilities.heading("Display All Shapes", 38, '*'));

        for(Shape s : shapes){
            System.out.println(s);
        }
    }

    /*
     * Prune the Non-Existent--i.e., null--shapes
     * from the array
     *
     * @param shapes the array to prune
     * @param count the size of the array
     * 
     * @return the new array size
     */
    static Shape[] pruneNullPtr(Shape[] shapes)
    {
        int non_null = 0;

        // Copy all non null pointers to a new array
        Shape[] pruned = new Shape[ shapes.length ];

        for(int i = 0; i < shapes.length; i++){
            if(shapes[i] != null){
                pruned[non_null] = shapes[i];
                non_null++; 
            }
        }

        // Create an a new array of size non_null
        shapes = new Shape[ non_null ];

        // Copy the elements from the pruned array
        for(int i = 0; i < non_null; i++){
            shapes[i] = pruned[i];
        }

        //Return the new array
        return shapes;
    }
}