package edu.odu.cs.cs330.examples.shapes;

import java.util.Scanner;
import java.io.BufferedReader;

import java.util.Iterator;

public class ShapeIterator implements Iterator<Shape>
{
    private Scanner theScanner;

    public ShapeIterator(BufferedReader buffer)
    {
        this.theScanner = new Scanner(buffer);
    }

    @Override
    public boolean hasNext()
    {
        return theScanner.hasNext();
    }

    @Override
    public Shape next()
    {
        String line   = theScanner.nextLine();
        int    sIndex = line.indexOf(';', 0);
        String name   = line.substring(0, sIndex); // [0, sIndex)

        Scanner lineScanner = new Scanner(line.substring(sIndex + 1,
                                                         line.length()));

        Shape shp = null;
        try {
            shp = ShapeFactory.createShape(name);
        }
        catch (CloneNotSupportedException exp) {
            shp = null;
        }

        if (shp != null) {
            shp.read(lineScanner);
        }

        return shp;
    }

    @Override
    public void remove()
        throws UnsupportedOperationException
    {
        throw new UnsupportedOperationException("Not Implemented");
    }
}
