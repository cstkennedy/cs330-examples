//Programmer : Thomas Kennedy

import java.util.ArrayList;
import java.util.Iterator;

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
        ArrayList<Shape> shapes = new ArrayList<Shape>();
        int              size   = 0; // original size of shapes container

        addShape(shapes, ShapeFactory.createShape("Triangle"));
        addShape(shapes, ShapeFactory.createShape("Right Triangle"));
        addShape(shapes, ShapeFactory.createShape("Equilateral Triangle"));
        addShape(shapes, ShapeFactory.createShape("Square"));
        addShape(shapes, ShapeFactory.createShape("Circle"));
        addShape(shapes, ShapeFactory.createShape("1337 Haxor"));

        size = shapes.size();
        
        System.out.println(Utilities.heading("Shapes That Exist", 38, '*'));
        System.out.printf("%-24s: %4d\n", "Original Size", size);     
        System.out.printf("%-24s: %4d\n", "Invalid Shapes", (size - shapes.size()));
        System.out.printf("%-24s: %4d\n", "New Size", shapes.size());
        
        System.out.println();

        // Print all the shapes
        System.out.println(Utilities.heading("Display All Shapes", 38, '*'));

        for(Shape s : shapes){
            System.out.println(s);
        }

        // Using an iterator

        System.out.println();
        System.out.println(Utilities.horizontalLine('~', 38));

        // C++ Container<Shape*>::iterator it = shapes.begin()
        Iterator<Shape> it = shapes.iterator(); 

        // C++ while (it != shapes.end())
        while (it.hasNext()) {
            Shape s = it.next();
            System.out.println(s);
        }
    }

    /**
     * Add a Shape iff it is not null
     *
     * @param shapes container of Shapes
     * @param toAdd shape "reference" to add
     */
    private static void addShape(ArrayList<Shape> shapes, Shape toAdd)
    {
        if (toAdd != null) {
            shapes.add(toAdd);
        }
    }
}