package house;

import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import static org.junit.jupiter.api.Assertions.*;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.*;
// import org.hamcrest.core.IsNull;

import java.util.Iterator;

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
public class TestRoom
{
    @Test
    public void testDefaultConstructor()
    {
        Room rm = new Room();

        assertThat(rm.getName(), equalTo("Generic"));
        assertThat(rm.getDimensions().getWidth(), closeTo(1.0, 1e-6));
        assertThat(rm.getDimensions().getLength(), closeTo(1.0, 1e-6));
        assertThat(rm.getFlooring().type, equalTo("Generic"));
        assertThat(rm.getFlooring().unitCost, closeTo(1.0, 1e-6));

        assertThat(rm.area(), closeTo(1.0, 1e-6));
        assertThat(rm.flooringCost(), closeTo(1.0, 1e-6));
    }

    @Test
    public void testNonDefaultConstructor1()
    {
        Room rm = new Room(2.0, 5.0, 27.0);

        assertThat(rm.getName(), equalTo("Generic"));
        assertThat(rm.getDimensions().getLength(), closeTo(2.0, 1e-6));
        assertThat(rm.getDimensions().getWidth(), closeTo(5.0, 1e-6));
        assertThat(rm.getFlooring().type, equalTo("Generic"));
        assertThat(rm.getFlooring().unitCost, closeTo(27.0, 1e-6));

        assertThat(rm.area(), closeTo(10.0, 1e-6));
        assertThat(rm.flooringCost(), closeTo(270.0, 1e-6));
    }

    @Test
    public void testNonDefaultConstructor2()
    {
        Room rm = new Room("Garage Workshop", 20.0, 30.0, 10.0);

        assertThat(rm.getName(), equalTo("Garage Workshop"));
        assertThat(rm.getDimensions().getLength(), closeTo(20.0, 1e-6));
        assertThat(rm.getDimensions().getWidth(), closeTo(30.0, 1e-6));
        assertThat(rm.getFlooring().type, equalTo("Generic"));
        assertThat(rm.getFlooring().unitCost, closeTo(10.0, 1e-6));

        assertThat(rm.area(), closeTo(600.0, 1e-6));
        assertThat(rm.flooringCost(), closeTo(6000, 1e-6));
    }

    @Test
    public void testNonDefaultConstructor3()
    {
        Room rm = new Room("Dream Garage Workshop", 40.0, 30.0, 10.0);

        assertThat(rm.getName(), equalTo("Dream Garage Workshop"));
        assertThat(rm.getDimensions().getLength(), closeTo(40.0, 1e-6));
        assertThat(rm.getDimensions().getWidth(), closeTo(30.0, 1e-6));
        assertThat(rm.getFlooring().type, equalTo("Generic"));
        assertThat(rm.getFlooring().unitCost, closeTo(10.0, 1e-6));

        assertThat(rm.area(), closeTo(1200.0, 1e-6));
        assertThat(rm.flooringCost(), closeTo(12000, 1e-6));
    }

    @Test
    public void testNonDefaultConstructor4()
    {
        Room rm = new Room("Dream Garage Workshop v2",
                           new Room.DimensionSet(40.0, 30.0),
                           20.0,
                           "High-End Epoxy");

        assertThat(rm.getName(), equalTo("Dream Garage Workshop v2"));
        assertThat(rm.getDimensions().getLength(), closeTo(40.0, 1e-6));
        assertThat(rm.getDimensions().getWidth(), closeTo(30.0, 1e-6));
        assertThat(rm.getFlooring().type, equalTo("High-End Epoxy"));
        assertThat(rm.getFlooring().unitCost, closeTo(20.0, 1e-6));

        assertThat(rm.area(), closeTo(1200.0, 1e-6));
        assertThat(rm.flooringCost(), closeTo(24000, 1e-6));
    }

    @Test
    public void testSetName()
    {
        Room rm = new Room();
        rm.setName("Linen Closet");

        assertThat(rm.getName(), equalTo("Linen Closet"));
        assertThat(rm.getDimensions().getWidth(), closeTo(1.0, 1e-6));
        assertThat(rm.getDimensions().getLength(), closeTo(1.0, 1e-6));
        assertThat(rm.getFlooring().type, equalTo("Generic"));
        assertThat(rm.getFlooring().unitCost, closeTo(1.0, 1e-6));

        assertThat(rm.area(), closeTo(1.0, 1e-6));
        assertThat(rm.flooringCost(), closeTo(1.0, 1e-6));
    }

    @Test
    public void testSetDimensions()
    {
        Room rm = new Room();
        rm.setDimensions(2, 3);

        assertThat(rm.getName(), equalTo("Generic"));
        assertThat(rm.getDimensions().getWidth(), closeTo(3.0, 1e-6));
        assertThat(rm.getDimensions().getLength(), closeTo(2.0, 1e-6));
        assertThat(rm.getFlooring().type, equalTo("Generic"));
        assertThat(rm.getFlooring().unitCost, closeTo(1.0, 1e-6));

        assertThat(rm.area(), closeTo(6.0, 1e-6));
        assertThat(rm.flooringCost(), closeTo(6.0, 1e-6));
    }

    @Test
    public void testSetFlooring()
    {
        Room rm = new Room();
        rm.setFlooring("Tile", 1.5);

        assertThat(rm.getName(), equalTo("Generic"));
        assertThat(rm.getDimensions().getWidth(), closeTo(1.0, 1e-6));
        assertThat(rm.getDimensions().getLength(), closeTo(1.0, 1e-6));
        assertThat(rm.getFlooring().type, equalTo("Tile"));
        assertThat(rm.getFlooring().unitCost, closeTo(1.5, 1e-6));

        assertThat(rm.area(), closeTo(1.0, 1e-6));
        assertThat(rm.flooringCost(), closeTo(1.5, 1e-6));
    }
}
