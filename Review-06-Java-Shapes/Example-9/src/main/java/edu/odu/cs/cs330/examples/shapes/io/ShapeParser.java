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
import edu.odu.cs.cs330.examples.shapes.ShapeFactory;

public class ShapeParser
{
    private ShapeParser()
    {
    }

    /**
     * Create the appropriate Shape class--e.g., Tool, Armour or Consumable.
     * <p>
     * How is **inheritance** used?
     *
     * @param scanner input from which to read in the Shape
     *
     * @return an initialized Shape object, or null
     */
    public static Shape parseShapeLine(String line)
    {
        /*
        String[] splitLine = line.strip().split(" ");

        String keyword = splitLine[0].strip();

        if (ShapeFactory.isNotKnown(keyword)) {
            return null;
        }

        String tokens[] = Arrays.stream(splitLine)
            .skip(1)
            .toArray(String[]::new);

        if (tokens.length != ShapeFactory.getNumberOfRequiredValues(keyword)) {
            return null;
        }

        Shape item = ShapeFactory.createItemFromTokens(keyword, tokens);

        return item;
        */
        return null;
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
            .map(line -> parseShapeLine(line))
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
        /*
        FileReader inFile = new FileReader(filename);
        BufferedReader buffer = new BufferedReader(inFile);

        List<Shape> itemsToStore = ShapeParser.readShapes(buffer);
        buffer.close();

        return itemsToStore;
        */
        return null;
    }
}
