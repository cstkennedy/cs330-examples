package shapes;

import org.junit.FixMethodOrder;
import org.junit.runners.MethodSorters;
import org.junit.Test;
import org.junit.Before;

import static org.junit.Assert.*;

import static org.hamcrest.CoreMatchers.*;
import org.hamcrest.core.IsNull;
import static org.hamcrest.number.IsCloseTo.closeTo;

import java.util.List;
import java.util.Arrays;

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
@FixMethodOrder(MethodSorters.NAME_ASCENDING)
public class TestShapeFactory {

    private List<String> knownNames;

    private int numberKnown;

    @Before
    public void setUp()
    {
        this.knownNames = Arrays.asList("Circle",
                                        "Square",
                                        "Triangle",
                                        "Equilateral Triangle",
                                        "Right Triangle");

        this.numberKnown = this.knownNames.size();
    }

    /**
     * Create a known valid Shape
     */
    @Test
    public void testCreateShapeSuccess()
    {
        assertThat(ShapeFactory.createShape("Circle"), is(notNullValue()));
    }

    /**
     * Try to create a known invalid Shape
     */
    @Test
    public void testcreateShapeFailure()
    {
        assertThat(ShapeFactory.createShape("Lol Nope"), is(nullValue()));
    }

    /**
     * Create a known valid Shape
     */
    @Test
    public void testIsKnownSuccess()
    {
        assertThat(ShapeFactory.isKnown("Circle"), is(true));
    }

    /**
     * Try to create a known invalid Shape
     */
    @Test
    public void testIsKnownFailure()
    {
        assertThat(ShapeFactory.isKnown("Lol Nope"), is(false));
    }

    @Test
    public void testListKnown()
    {
        String knownStr = ShapeFactory.listKnown();

        for (String name : this.knownNames) {
            assertThat(knownStr, containsString(name));
        }
    }

    @Test
    public void testNumberKnown()
    {
        assertThat(ShapeFactory.numberKnown(), equalTo(this.numberKnown));
    }
}
