package edu.odu.cs.cs330.examples.shapes.io;

import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import static org.junit.jupiter.api.Assertions.*;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.*;
import org.hamcrest.core.IsNull;

import java.io.StringReader;
import java.io.BufferedReader;
import java.io.IOException;
import java.util.Scanner;
import java.util.List;
import java.util.Iterator;

import edu.odu.cs.cs330.examples.shapes.Shape;
import edu.odu.cs.cs330.examples.shapes.Circle;
import edu.odu.cs.cs330.examples.shapes.Square;
import edu.odu.cs.cs330.examples.shapes.Triangle;
import edu.odu.cs.cs330.examples.shapes.EquilateralTriangle;
import edu.odu.cs.cs330.examples.shapes.RightTriangle;


/**
 * 1 - Does this piece of code perform the operations
 *     it was designed to perform?
 *
 * 2 - Does this piece of code do something it was not
 *     designed to perform?
 *
 * 1 Test per mutator
 *
 * This is technically an Integration Test.
 */
@TestMethodOrder(MethodOrderer.MethodName.class)
public class TestShapeParser
{
    final String CIRCLE_INPUT = "Circle; 5";
    final String SQUARE_INPUT = "Square; 9";
    final String EQL_TRIANGLE_INPUT = "Equilateral Triangle; 3";
    final String RIGHT_TRIANGLE_INPUT = "Right Triangle; 4 5";
    final String TRIANGLE_INPUT = "Triangle; 5 7 9";
    final String INVALID_INPUT = "1337 Haxor; 1 lol i broke it";

    final String ALL_VALID = String.join(
        "\n",
        CIRCLE_INPUT,
        SQUARE_INPUT,
        EQL_TRIANGLE_INPUT,
        RIGHT_TRIANGLE_INPUT,
        TRIANGLE_INPUT,
        INVALID_INPUT
    );

    /*
    Consumable foodShape;

    @BeforeEach
    public void setUp()
    {
        foodShape = new Consumable();
        foodShape.setName("Green-Tea");
        foodShape.setEffect("Wake-Up");
        foodShape.setNumberOfUses(5);
    }

    @Test
    public void testParseShapeLineSuccess()
    {
        String inputStr = "Food Green-Tea Wake-Up 5";
        Shape item = ShapeParser.parseItemLine(inputStr);

        assertThat(item, equalTo(foodShape));
        assertThat(item.toString(), equalTo(foodShape.toString()));
    }

    @Test
    public void testParseShapeLineFailUnknownItem()
    {
        String inputStr = "NOTACTUALLY-Food Green-Tea Wake-Up 5";
        Shape item = ShapeParser.parseItemLine(inputStr);

        assertThat(item, is(nullValue()));
    }

    @Test
    public void testParseShapeLineFailMalformedLine()
    {
        //Too Many Shapes
        String inputStr = "Food Green-Tea Wake-Up 5 Too Many Shapes";
        Shape item = ShapeParser.parseItemLine(inputStr);
        assertThat(item, is(nullValue()));

        // Too Few Shapes
        inputStr = "Food Green-Tea Wake-Up";
        item = ShapeParser.parseShapeLine(inputStr);
        assertThat(item, is(nullValue()));
    }

    @Test
    public void readShapes()
        throws IOException
    {
        // Set up test data
        String src = String.join(
            "\n",
            "Armor Boots Diamond 100 10 FeatherFalling 4 lightning",
            "Food Tomato Hunger-10 2",
            "LOLNOTAVALIDITEM potato 7",
            "Tool Shovel Gold 20 3 Unbreaking 2"
        );

        Armour boots = new Armour();
        boots.fromTokens(new String[] {
            "Boots", "Diamond", "100", "10", "FeatherFalling", "4", "lightning"
        });

        Consumable tomato = new Consumable();
        tomato.setName("Tomato");
        tomato.setEffect("Hunger-10");
        tomato.setNumberOfUses(2);

        Tool shovel = new Tool();
        shovel.fromTokens(new String[] {
            "Shovel", "Gold", "20", "3", "Unbreaking", "2"
        });

        // Run the test
        BufferedReader buffer = new BufferedReader(new StringReader(src));
        List<Shape> items = ShapeParser.readItems(buffer);

        assertThat(items.size(), equalTo(3));

        Iterator<Shape> it = items.iterator();
        assertThat(it.next(), equalTo(boots));
        assertThat(it.next(), equalTo(tomato));
        assertThat(it.next(), equalTo(shovel));
    }

    @Test
    public void readShapesFromFile()
        throws IOException
    {
        // Set up test data
        String dataFile = "src/test/resources/test-items-01.txt";

        Armour boots = new Armour();
        boots.fromTokens(new String[] {
            "Boots", "Diamond", "100", "10", "FeatherFalling", "4", "lightning"
        });

        Consumable tomato = new Consumable();
        tomato.fromTokens(new String[]{
            "Tomato", "Hunger-10", "2"
        });

        Tool shovel = new Tool();
        shovel.fromTokens(new String[] {
            "Shovel", "Gold", "20", "3", "Unbreaking", "2"
        });

        // Run the test
        List<Shape> items = ShapeParser.readItemsFromFile(dataFile);

        assertThat(items.size(), equalTo(3));

        Iterator<Shape> it = items.iterator();
        assertThat(it.next(), equalTo(boots));
        assertThat(it.next(), equalTo(tomato));
        assertThat(it.next(), equalTo(shovel));
    }

    @Test
    public void testReadCircle()
    {
        ShapeIterator sIterator = generateShapeIterator(CIRCLE_INPUT);

        assertTrue(sIterator.hasNext());

        Shape shp = sIterator.next();

        assertThat(shp, is(instanceOf(Circle.class)));
        assertThat(shp.name(), is(equalTo("Circle")));
        assertThat(((Circle) shp).radius(), is(closeTo(5, 1e-8)));
    }

    @Test
    public void testReadSquare()
    {
        ShapeIterator sIterator = generateShapeIterator(SQUARE_INPUT);

        assertTrue(sIterator.hasNext());

        Shape shp = sIterator.next();

        assertThat(shp, is(instanceOf(Square.class)));
        assertThat(shp.name(), is(equalTo("Square")));
        assertThat(((Square) shp).side(), is(closeTo(9, 1e-8)));
    }

    @Test
    public void testReadEquilateralTriangle()
    {
        ShapeIterator sIterator = generateShapeIterator(EQL_TRIANGLE_INPUT);

        assertTrue(sIterator.hasNext());

        Shape shp = sIterator.next();

        assertThat(shp, is(instanceOf(EquilateralTriangle.class)));
        assertThat(shp.name(), is(equalTo("Equilateral Triangle")));
        assertThat(((EquilateralTriangle) shp).side(), is(closeTo(3, 1e-8)));
    }

    @Test
    public void testReadRightTriangle()
    {

        ShapeIterator sIterator = generateShapeIterator(RIGHT_TRIANGLE_INPUT);

        assertTrue(sIterator.hasNext());

        Shape shp = sIterator.next();

        assertThat(shp, is(instanceOf(RightTriangle.class)));
        assertThat(shp.name(), is(equalTo("Right Triangle")));

        RightTriangle tri = (RightTriangle) shp;
        assertThat(tri.base(), is(closeTo(4, 1e-8)));
        assertThat(tri.height(), is(closeTo(5, 1e-8)));
    }

    @Test
    public void testReadTriangle()
    {
        ShapeIterator sIterator = generateShapeIterator(TRIANGLE_INPUT);

        assertTrue(sIterator.hasNext());

        Shape shp = sIterator.next();

        assertThat(shp, is(instanceOf(Triangle.class)));
        assertThat(shp.name(), is(equalTo("Triangle")));

        Triangle tri = (Triangle) shp;
        assertThat(tri.sideA(), is(closeTo(5, 1e-8)));
        assertThat(tri.sideB(), is(closeTo(7, 1e-8)));
        assertThat(tri.sideC(), is(closeTo(9, 1e-8)));
    }

    @Test
    public void testReadInvalidShape()
    {
        ShapeIterator sIterator = generateShapeIterator(INVALID_INPUT);

        assertFalse(sIterator.hasNext());

        Shape shp = sIterator.next();

        assertThat(shp, is(nullValue()));
    }

    @Test
    public void readMultipleShapes()
    {

        final String long_input = "Triangle; 4 4 4\n"
                                + "Right Triangle; 4 5\n"
                                + "Equilateral Triangle; 3\n"
                                + "Square; 9\n"
                                + "Circle; 5\n"
                                + "1337 Haxor; 1 lol i broke it\n";

        ShapeIterator sIterator = generateShapeIterator(long_input);

        assertThat(sIterator.hasNext(), is(true));
        assertThat(sIterator.next(), is(instanceOf(Triangle.class)));

        assertThat(sIterator.hasNext(), is(true));
        assertThat(sIterator.next(), is(instanceOf(RightTriangle.class)));

        assertThat(sIterator.hasNext(), is(true));
        assertThat(sIterator.next(), is(instanceOf(EquilateralTriangle.class)));

        assertThat(sIterator.hasNext(), is(true));
        assertThat(sIterator.next(), is(instanceOf(Square.class)));

        assertThat(sIterator.hasNext(), is(true));
        assertThat(sIterator.next(), is(instanceOf(Circle.class)));

        assertThat(sIterator.hasNext(), is(false));
        assertThat(sIterator.next(), is(nullValue()));
        assertThat(sIterator.hasNext(), is(false));
    }
    */
}

