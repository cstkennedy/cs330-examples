import java.util.ArrayList;
import java.util.Iterator;

import utilities.Utilities;
import static utilities.Utilities.projectHeading;
import static utilities.Utilities.heading;
import static utilities.Utilities.projectHeading;
import static utilities.Utilities.horizontalLine;

import shapes.*;


public class RunShapes {
    static final String[] PROGRAM_HEADING = {
        "Objects & Inheritance: 2-D Shapes",
        "Thomas Kennedy"
    };  ///< Program Title

    public static void main(String[] args)
    {
        // Print main program heading
        System.out.println(projectHeading(PROGRAM_HEADING, Utilities.W_WIDTH));

        // Examine the ShapeFactory
        System.out.println(heading("Available Shapes", 38, '*'));

        // List the available shapes
        System.out.print(ShapeFactory.listKnown());
        System.out.println(horizontalLine('-', 38));
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

        System.out.println(heading("Shapes That Exist", 38, '*'));
        System.out.printf("%-24s: %4d\n", "Original Size", size);
        System.out.printf("%-24s: %4d\n", "Invalid Shapes", (size - shapes.size()));
        System.out.printf("%-24s: %4d\n", "New Size", shapes.size());

        System.out.println();

        // Print all the shapes
        System.out.println(heading("Display All Shapes", 38, '*'));

        for (Shape s : shapes) {
            System.out.println(s);
        }

        // Using an iterator

        System.out.println();
        System.out.println(horizontalLine('~', 38));

        // C++ Container<Shape*>::iterator it = shapes.begin()
        Iterator<Shape> it = shapes.iterator();

        // C++ while (it != shapes.end())
        while (it.hasNext()) {
            Shape s = it.next();
            System.out.println(s);
            // System.out.println(it.next());
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
