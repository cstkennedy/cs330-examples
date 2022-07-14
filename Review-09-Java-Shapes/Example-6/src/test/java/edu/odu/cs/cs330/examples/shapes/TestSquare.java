package edu.odu.cs.cs330.examples.shapes;

import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import static org.junit.jupiter.api.Assertions.*;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.*;
import org.hamcrest.core.IsNull;
import static org.hamcrest.number.IsCloseTo.closeTo;

import java.io.StringReader;
import java.util.Scanner;

/**
 * 1 - Does this piece of code perform the operations
 *     it was designed to perform?
 *
 * 2 - Does this piece of code do something it was not
 *     designed to perform?
 *
 * 1 Test per mutator
 */
@TestMethodOrder(MethodOrderer.MethodName.class)
public class TestSquare {

    private Square generic;
    private Square fancy;

    @BeforeEach
    public void setUp()
    {
        this.generic = new Square();
        this.fancy   = new Square(2);
    }

    @Test
    public void testDefaultConstructor()
    {
        assertThat(this.generic.name(), equalTo("Square"));
        assertThat(this.generic.side(), closeTo(1, 1e-8));
    }

    @Test
    public void testConstructor()
    {
        assertThat(this.fancy.name(), equalTo("Square"));
        assertThat(this.fancy.side(), closeTo(2.0, 1e-8));
    }

    @Test
    public void testSideSetter()
    {
        Square aSquare = (Square) new Square();

        aSquare.side(7.39);

        assertThat(aSquare.side(), closeTo(7.39, 1e-8));
        assertThat(aSquare.name(), equalTo("Square"));
    }

    @Test
    public void testArea()
    {
        assertThat(this.generic.area(),
                   closeTo(Math.pow(this.generic.side(), 2), 1e-8));

        assertThat(this.fancy.area(),
                   closeTo(Math.pow(this.fancy.side(), 2), 1e-8));
    }

    @Test
    public void testPerimeter()
    {
        assertThat(this.generic.perimeter(),
                   closeTo(4 * this.generic.side(), 1e-8));

        assertThat(this.fancy.perimeter(),
                   closeTo(4 * this.fancy.side(), 1e-8));
    }

    @Test
    public void testClone()
        throws CloneNotSupportedException
    {
        Square aCopy = (Square) this.fancy.clone();

        assertThat(aCopy, is(not(sameInstance(this.fancy))));

        // I really should have defined __eq__
        assertThat(aCopy.side(), closeTo(this.fancy.side(), 0.001));
    }

    @Test
    public void testToString()
    {
        String fancyStr = this.fancy.toString();

        assertThat(fancyStr, startsWith("Name"));
        assertThat(fancyStr, containsString("Square"));
        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f",
                                                "Perimeter",
                                                this.fancy.perimeter())));

        assertThat(fancyStr,
                    containsString(String.format("%-12s: %24.4f",
                                                 "Area",
                                                  this.fancy.area())));

        assertThat(fancyStr,
                    containsString(String.format("%-12s: %24.4f",
                                                 "Side",
                                                 this.fancy.side())));

        assertThat(fancyStr, endsWith("\n"));
    }

    @Test
    public void testRead()
    {
        Scanner snr = new Scanner(new StringReader("9.7"));

        Square aSquare = new Square();
        aSquare.read(snr);

        assertThat(aSquare.name(), equalTo("Square"));
        assertThat(aSquare.side(), closeTo(9.7, 0.01));
    }
}
