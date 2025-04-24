package edu.odu.cs.cs330.examples.shapes.io;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.Scanner;

import edu.odu.cs.cs330.examples.shapes.Shape;
import edu.odu.cs.cs330.examples.shapes.TraitFromDimensions;
import edu.odu.cs.cs330.examples.shapes.ShapeFactory;

public class ShapeParser
{
    private ShapeParser()
    {
    }

    private static <T extends TraitFromDimensions> T initFromDims(T obj, String restOfLine)
    {
        double[] dims = new double[obj.numDims()];

        Scanner snr = new Scanner(restOfLine);

        for (int i = 0; i < dims.length; ++i) {
            dims[i] = snr.nextDouble();
        }

        obj.createFromDims(dims);

        return obj;
    }

    /**
     * Create the appropriate Shape class--e.g., Tool, Armour or Consumable.
     * <p>
     * How is **inheritance** used?
     *
     * @param splitLine raw input line split on whitespace (e.g., ' ', '\t')
     *
     * @return an initialized Shape object, or null
     */
    public static Shape parseShapeLine(String[] splitLine)
    {
        final String name = splitLine[0];

        Shape shp = null;
        try {
            shp = ShapeFactory.createShape(name);
        }
        catch (CloneNotSupportedException exp) {
            return null;
        }

        String restOfLine = splitLine[1];
        shp = (Shape) ShapeParser.initFromDims(shp, restOfLine);

        return shp;
    }

    /**
     * Read an input stream and generate a collection of Shapes.
     *
     * @param ins source from which to read Shapes
     *
     * @return initialized list of Shapes
     *
     * @throws IOException if an input error occurs
     */
    public static List<Shape> readShapes(BufferedReader ins)
        throws IOException
    {
        return ins
            .lines()
            .filter(line -> line.indexOf(";") > 0)
            .map(line -> line.split(";"))
            .filter(splitLine -> ShapeFactory.isKnown(splitLine[0]))
            .map(splitLine -> parseShapeLine(splitLine))
            .filter(Objects::nonNull)
            .collect(java.util.stream.Collectors.toList());
    }

    /**
     * Read an input stream and generate a collection of Shapes.
     *
     * @param filename source from which to read Shapes
     *
     * @return initialized list of Shapes
     *
     * @throws IOException if an input error occurs
     */
    public static List<Shape> readShapesFromFile(String filename)
        throws IOException
    {
        FileReader inFile = new FileReader(filename);
        BufferedReader buffer = new BufferedReader(inFile);

        List<Shape> shapesToStore = ShapeParser.readShapes(buffer);
        buffer.close();

        return shapesToStore;
    }
}
