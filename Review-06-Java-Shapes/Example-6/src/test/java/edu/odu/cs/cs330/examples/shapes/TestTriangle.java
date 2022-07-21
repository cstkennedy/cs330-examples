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
public class TestTriangle
{
    Triangle generic;
    Triangle fancy;

    @BeforeEach
    public void setUp()
    {
        this.generic = new Triangle();
        this.fancy = new Triangle(1, 1, Math.sqrt(2));
    }

    @Test
    public void testDefaultConstructor()
    {
        assertThat(this.generic.name(), equalTo("Triangle"));
        assertThat(this.generic.sideA(), closeTo(1, 1e-8));
        assertThat(this.generic.sideB(), closeTo(1, 1e-8));
        assertThat(this.generic.sideC(), closeTo(1, 1e-8));
    }

    @Test
    public void testConstructor()
    {
        assertThat(this.fancy.name(), equalTo("Triangle"));
        assertThat(this.fancy.sideA(), closeTo(1, 1e-8));
        assertThat(this.fancy.sideB(), closeTo(1, 1e-8));
        assertThat(this.fancy.sideC(), closeTo(Math.sqrt(2.0), 1e-8));
    }

    @Test
    public void testSideASetter()
    {
        Triangle aTriangle = new Triangle();

        aTriangle.sideA(7.39);

        assertThat(aTriangle.sideA(), closeTo(7.39, 1e-8));
        assertThat(aTriangle.sideB(), closeTo(1, 1e-8));
        assertThat(aTriangle.sideC(), closeTo(1, 1e-8));
    }

    @Test
    public void testSideBSetter()
    {
        Triangle aTriangle = new Triangle();

        aTriangle.sideB(7.39);

        assertThat(aTriangle.sideA(), closeTo(1, 1e-8));
        assertThat(aTriangle.sideB(), closeTo(7.39, 1e-8));
        assertThat(aTriangle.sideC(), closeTo(1, 1e-8));
    }

    @Test
    public void testSideCSetter()
    {
        Triangle aTriangle = new Triangle();

        aTriangle.sideC(7.39);

        assertThat(aTriangle.sideA(), closeTo(1, 1e-8));
        assertThat(aTriangle.sideB(), closeTo(1, 1e-8));
        assertThat(aTriangle.sideC(), closeTo(7.39, 1e-8));
    }

    @Test
    public void testArea()
    {
        // Based on 1/2 base * height (base=1, height = sqrt(3)/2);
        double expectedArea = Math.sqrt(3.0) / 4;

        assertThat(this.generic.area(), closeTo(expectedArea, 1e-8));

        // Based on 1/2 base * height (base=1, height = 1);
        expectedArea = 0.5;

        assertThat(this.fancy.area(), closeTo(expectedArea, 1e-8));
    }

    @Test
    public void testPerimeter()
    {
        double expectedP = this.generic.sideA()
                         + this.generic.sideB()
                         + this.generic.sideC();

        assertThat(this.generic.perimeter(), closeTo(expectedP, 1e-8));

        expectedP = this.fancy.sideA()
                  + this.fancy.sideB()
                  + this.fancy.sideC();

        assertThat(this.fancy.perimeter(), closeTo(expectedP, 1e-8));
    }

    @Test
    public void testClone()
        throws CloneNotSupportedException
    {
        Triangle aCopy = (Triangle) this.fancy.clone();

        assertThat(aCopy, is(not(sameInstance(this.fancy))));

        // I really should have defined __eq__
        assertThat(aCopy.sideA(), closeTo(this.fancy.sideA(), 1e-8));
        assertThat(aCopy.sideB(), closeTo(this.fancy.sideB(), 1e-8));
        assertThat(aCopy.sideC(), closeTo(this.fancy.sideC(), 1e-8));
    }

    @Test
    public void testToString()
    {
        String fancyStr = this.fancy.toString();

        assertThat(fancyStr, startsWith("Name"));
        assertThat(fancyStr, containsString("Triangle"));

        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f", "Perimeter", this.fancy.perimeter())));

        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f", "Area", this.fancy.area())));

        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f", "Side A", this.fancy.sideA())));

        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f", "Side B", this.fancy.sideB())));

        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f", "Side C", this.fancy.sideC())));

        assertThat(fancyStr, endsWith("\n"));
    }

    @Test
    public void testRead()
    {
        Scanner snr = new Scanner(new StringReader("9.7 5.2 2.1"));

        Triangle aTriangle = new Triangle();
        aTriangle.read(snr);

        assertThat(aTriangle.name(), equalTo("Triangle"));
        assertThat(aTriangle.sideA(), closeTo(9.7, 0.01));
        assertThat(aTriangle.sideB(), closeTo(5.2, 0.01));
        assertThat(aTriangle.sideC(), closeTo(2.1, 0.01));
    }
}
