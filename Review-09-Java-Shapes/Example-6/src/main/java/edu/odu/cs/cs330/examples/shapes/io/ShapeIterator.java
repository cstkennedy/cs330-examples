package edu.odu.cs.cs330.examples.shapes.io;

import java.util.Scanner;
import java.io.BufferedReader;
import java.io.IOException;

import java.util.Iterator;

import edu.odu.cs.cs330.examples.shapes.Shape;
import edu.odu.cs.cs330.examples.shapes.ShapeFactory;

/**
 * Iterator over a buffer of Shape objects. This iterator stops when
 * Buffer is exhausted. Invalid Shape objects (null) are skipped.
 */
public class ShapeIterator implements Iterator<Shape>
{
    /**
     * Source buffer from which Shape objects are read.
     */
    private BufferedReader theBuffer;

    /**
     * Store the next pre-read Shape.
     */
    private Shape queued;

    /**
     * Construct a ShapeIterator over an input buffer.
     *
     * @param buffer shape data buffer
     */
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
        String  line        = theBuffer.readLine();
        Scanner lineScanner = null;

        Shape shp = null;

        while (shp == null && line != null) {
            final int    sIndex = line.indexOf(';', 0);
            final String name   = line.substring(0, sIndex); // [0, sIndex)

            shp = ShapeFactory.createShape(name);

            if (shp != null) {
                lineScanner = new Scanner(line.substring(sIndex + 1,
                                                         line.length()));
                shp.read(lineScanner);
            }
            else {
                line = theBuffer.readLine();
            }
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
        catch (IOException | CloneNotSupportedException exp) {
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
