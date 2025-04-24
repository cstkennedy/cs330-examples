package shapes;

import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import static org.junit.jupiter.api.Assertions.*;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.*;
import org.hamcrest.core.IsNull;
import static org.hamcrest.number.IsCloseTo.closeTo;

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
public class TestRightTriangle
{
    private RightTriangle generic;
    private RightTriangle fancy;

    @BeforeEach
    public void setUp()
    {
        this.generic = new RightTriangle();
        this.fancy = new RightTriangle(2, 3);
    }

    @Test
    public void testDefaultConstructor()
    {
        assertThat(this.generic.name(), equalTo("Right Triangle"));
        assertThat(this.generic.base(), closeTo(1, 1e-8));
        assertThat(this.generic.height(), closeTo(1, 1e-8));
        assertThat(this.generic.hypotenuse(), closeTo(Math.sqrt(2), 1e-8));
    }

    @Test
    public void testConstructor()
    {
        assertThat(this.fancy.name(), equalTo("Right Triangle"));
        assertThat(this.fancy.base(), closeTo(2, 1e-8));
        assertThat(this.fancy.height(), closeTo(3, 1e-8));
        assertThat(this.fancy.hypotenuse(), closeTo(Math.sqrt(13), 1e-8));
    }

    @Test
    public void testBaseSetter()
    {
        RightTriangle aTriangle = new RightTriangle();

        aTriangle.base(7.39);

        assertThat(aTriangle.base(), closeTo(7.39, 1e-8));
        assertThat(aTriangle.height(), closeTo(1, 1e-8));
        assertThat(aTriangle.hypotenuse(), closeTo(7.4573, 1e-4));
    }

    @Test
    public void testHeightSetter()
    {
        RightTriangle aTriangle = new RightTriangle();

        aTriangle.height(7.39);

        assertThat(aTriangle.base(), closeTo(1, 1e-8));
        assertThat(aTriangle.height(), closeTo(7.39, 1e-8));
        assertThat(aTriangle.hypotenuse(), closeTo(7.4573, 1e-4));
    }

    @Test
    public void testArea()
    {
        // Based on 1/2 base * height (base=1, height = 1);
        double expectedArea = 0.5;

        assertThat(this.generic.area(), closeTo(expectedArea, 1e-8));

        // Based on 1/2 base * height (base=2, height = 3);
        expectedArea = 3;

        assertThat(this.fancy.area(), closeTo(expectedArea, 1e-8));
    }

    @Test
    public void testPerimeter()
    {
        double expectedP = this.generic.base()
                         + this.generic.height()
                         + this.generic.hypotenuse();

        assertThat(this.generic.perimeter(), closeTo(expectedP, 1e-8));

        expectedP = this.fancy.base()
                  + this.fancy.height()
                  + this.fancy.hypotenuse();

        assertThat(this.fancy.perimeter(), closeTo(expectedP, 1e-8));
    }

    @Test
    public void testClone()
    {
        RightTriangle aCopy = (RightTriangle) this.fancy.clone();

        assertThat(aCopy, is(not(sameInstance(this.fancy))));

        // I really should have defined __eq__
        assertThat(aCopy.base(), closeTo(this.fancy.base(), 1e-8));
        assertThat(aCopy.height(), closeTo(this.fancy.height(), 1e-8));
        assertThat(aCopy.hypotenuse(), closeTo(this.fancy.hypotenuse(), 1e-8));
    }

    @Test
    public void testToString()
    {
        String fancyStr = this.fancy.toString();

        assertThat(fancyStr, startsWith("Name"));
        assertThat(fancyStr, containsString("Right Triangle"));
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
                                                "Base",
                                                 this.fancy.base())));
        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f",
                                                "Height",
                                                this.fancy.height())));
        assertThat(fancyStr,
                   containsString(String.format("%-12s: %24.4f",
                                                "Hypotenuse",
                                                this.fancy.hypotenuse())));
        assertThat(fancyStr, endsWith("\n"));
    }
}
