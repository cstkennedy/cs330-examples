package edu.odu.cs.cs330.examples.shapes;

import java.util.Scanner;
import java.io.BufferedReader;
import java.io.IOException;

import java.util.Iterator;

/**
 * Iterator over a buffer of Shape objects. This iterator stops when the
 * Buffer is exhausted or an invalid shape is encountered.
 */
public class ShapeIterator implements Iterator<Shape>
{
    /**
     * Source buffer from which Shape objects are read.
     */
    private BufferedReader theBuffer;

    /**
     * Store the next pre-read Shape
     */
    Shape queued;

    public ShapeIterator(BufferedReader buffer)
    {
        this.theBuffer = buffer;

        try {
            queued = readNext();
        }
        catch (IOException exp) {
            queued = null;
        }
        catch (CloneNotSupportedException exp) {
            queued = null;
        }
    }

    @Override
    public boolean hasNext()
    {
        return queued != null;
    }

    /**
     * Read the next Shape.
     *
     * @return next Shape
     *
     * @throws IOException if an error occurred reading from the buffer
     *
     * @throws CloneNotSupportedException if ShapeFactory can not clone a Shape
     */
    private Shape readNext()
        throws IOException, CloneNotSupportedException
    {
        String line   = theBuffer.readLine();
        int    sIndex = line.indexOf(';', 0);
        String name   = line.substring(0, sIndex); // [0, sIndex)

        Scanner lineScanner = new Scanner(line.substring(sIndex + 1,
                                                         line.length()));

        Shape shp = ShapeFactory.createShape(name);

        if (shp != null) {
            shp.read(lineScanner);
        }

        return shp;
    }

    @Override
    public Shape next()
    {
        Shape shp = queued;

        try {
            queued = readNext();
        }
        catch (IOException exp) {
            queued = null;
        }
        catch (CloneNotSupportedException exp) {
            queued = null;
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
