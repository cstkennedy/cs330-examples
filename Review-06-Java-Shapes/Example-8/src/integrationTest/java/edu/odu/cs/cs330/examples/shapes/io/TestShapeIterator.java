package edu.odu.cs.cs330.examples.shapes.io;

import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import static org.junit.jupiter.api.Assertions.*;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.*;
import org.hamcrest.core.IsNull;
import static org.hamcrest.number.IsCloseTo.closeTo;

import java.util.List;
import java.util.Arrays;
import java.io.BufferedReader;
import java.io.StringReader;

import edu.odu.cs.cs330.examples.shapes.*;

/**
 *   **This is technically a set of Integration Tests**
 *
 * 1 - Does this piece of code perform the operations
 *     it was designed to perform?
 *
 * 2 - Does this piece of code do something it was not
 *     designed to perform?
 *
 * 1 Test per mutator
 */
@TestMethodOrder(MethodOrderer.MethodName.class)
public class TestShapeIterator {

    final String CIRCLE_INPUT = "Circle; 5";
    final String SQUARE_INPUT = "Square; 9";
    final String EQL_TRIANGLE_INPUT = "Equilateral Triangle; 3";
    final String RIGHT_TRIANGLE_INPUT = "Right Triangle; 4 5";
    final String TRIANGLE_INPUT = "Triangle; 5 7 9";
    final String INVALID_INPUT = "1337 Haxor; 1 lol i broke it";

    @BeforeEach
    public void setUp()
    {
    }

    /**
     * Generate a ShapeIterator from a test string.
     *
     * @param srcStr test string
     */
    private ShapeIterator generateShapeIterator(String srcStr)
    {
        StringReader sReader = new StringReader(srcStr);
        BufferedReader bReader = new BufferedReader(sReader);

        return new ShapeIterator(bReader);
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
}
