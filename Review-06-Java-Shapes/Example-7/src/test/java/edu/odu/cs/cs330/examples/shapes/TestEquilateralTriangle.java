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
public class TestEquilateralTriangle
{
    private EquilateralTriangle generic;
    private EquilateralTriangle fancy;

    @BeforeEach
    public void setUp()
    {
        this.generic = new EquilateralTriangle();
        this.fancy = new EquilateralTriangle(3);
    }

    @Test
    public void testDefaultConstructor()
    {
        assertThat(this.generic.name(), equalTo("Equilateral Triangle"));
        assertThat(this.generic.side(), closeTo(1, 1e-8));
    }

    @Test
    public void testConstructor()
    {
        assertThat(this.fancy.name(), equalTo("Equilateral Triangle"));
        assertThat(this.fancy.side(), closeTo(3, 1e-8));
    }

    @Test
    public void testSideSetter()
    {
        EquilateralTriangle aTriangle = new EquilateralTriangle();

        aTriangle.side(7.39);
        assertThat(aTriangle.side(), closeTo(7.39, 1e-8));
    }

    @Test
    public void testArea()
    {
        // Based on 1/2 base * height (side=1)
        double expectedArea = Math.sqrt(3) / 4;

        assertThat(this.generic.area(),
                    closeTo(expectedArea, 1e-8));

        // Based on 1/2 side * height (side=3)
        expectedArea = 3 * Math.sqrt(27) / 4;
        assertThat(this.fancy.area(), closeTo(expectedArea, 1e-8));
    }

    @Test
    public void testPerimeter()
    {
        assertThat(this.generic.perimeter(), closeTo(3, 1e-8));
        assertThat(this.fancy.perimeter(), closeTo(9, 1e-8));
    }

    @Test
    public void testClone()
        throws CloneNotSupportedException
    {
        EquilateralTriangle aCopy = (EquilateralTriangle) this.fancy.clone();

        assertThat(aCopy, is(not(sameInstance(this.fancy))));

        // I really should have defined __eq__
        assertThat(aCopy.side(), closeTo(this.fancy.side(), 1e-8));
    }

    @Test
    public void testToString()
    {
        String fancyStr = this.fancy.toString();

        assertThat(fancyStr, startsWith("Name"));
        assertThat(fancyStr, containsString("Equilateral Triangle"));
        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f",
                                                "Perimeter",
                                                this.fancy.perimeter())));
        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f", "Area", this.fancy.area())));

        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f", "Side", this.fancy.side())));

        assertThat(fancyStr, endsWith("\n"));
    }
}
