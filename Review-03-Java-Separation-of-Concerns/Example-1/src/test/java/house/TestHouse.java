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
import java.util.List;
import java.util.ArrayList;
import java.util.stream.Collectors;

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
public class TestHouse
{
    private static final Room DEFAULT_ROOM = new Room();
    private static final List<Room> ROOMS = new ArrayList(List.of(
        new Room(),
        new Room("Garage Workshop", 20.0, 30.0, 10.0),
        new Room("Dream Garage Workshop", 40.0, 30.0, 10.0),
        new Room("Dream Garage Workshop v2",
                 new Room.DimensionSet(40.0, 30.0),
                 20.0,
                 "High-End Epoxy")
    ));

    private House emptyHouse;

    @BeforeEach
    public void setUp()
    {
        emptyHouse = new House();
    }

    @Test
    public void testDefaultConstructor()
    {
        assertThat(emptyHouse.getName(), equalTo("Generic"));
        assertThat(emptyHouse.size(), equalTo(0));

        assertThat(emptyHouse, equalTo(emptyHouse));

        assertThat(emptyHouse.iterator().hasNext(), is(false));
        assertThat(emptyHouse.toString(), containsString(emptyHouse.getName()));
    }

    @Test
    public void testConstructorWithName()
    {
        House namedHouse = new House("Creative Name");

        assertThat(namedHouse.getName(), equalTo("Creative Name"));
        assertThat(namedHouse.size(), equalTo(0));

        assertThat(namedHouse, equalTo(namedHouse));
        assertThat(namedHouse, not(equalTo(emptyHouse)));

        assertThat(namedHouse.iterator().hasNext(), is(false));
        assertThat(namedHouse.toString(), containsString(namedHouse.getName()));
    }

    @Test
    public void testSetName()
    {
        House namedHouse = new House("Creative Name");
        namedHouse.setName("New Creative Name");

        assertThat(namedHouse.getName(), equalTo("New Creative Name"));
        assertThat(namedHouse.size(), equalTo(0));

        assertThat(namedHouse, equalTo(namedHouse));
        assertThat(namedHouse, not(equalTo(emptyHouse)));

        assertThat(namedHouse.iterator().hasNext(), is(false));
        assertThat(namedHouse.toString(), containsString(namedHouse.getName()));
    }

    @Test
    public void testAddRoom1()
    {
        House house = new House();
        house.addRoom(ROOMS.get(0));

        assertThat(house.getName(), equalTo("Generic"));
        assertThat(house.size(), equalTo(1));

        assertThat(house, equalTo(house));
        assertThat(house, not(equalTo(emptyHouse)));

        assertThat(house.iterator().hasNext(), is(true));
        assertThat(house.toString(), stringContainsInOrder(ROOMS.get(0).toString()));

        // Test equals for two different single-room houses
        House anotherHouse = new House();
        anotherHouse.addRoom(ROOMS.get(2));

        assertThat(house, not(equalTo(anotherHouse)));
    }

    @Test
    public void testAddRooms()
    {
        House house = new House();

        for (Room room : ROOMS) {
            house.addRoom(room);
        }

        assertThat(house.getName(), equalTo("Generic"));
        assertThat(house.size(), equalTo(ROOMS.size()));

        assertThat(house, equalTo(house));
        assertThat(house, not(equalTo(emptyHouse)));

        assertThat(house.iterator().hasNext(), is(true));

        List<String> roomsAsStrings = ROOMS.stream()
                                        .map(Room::toString)
                                        .collect(Collectors.toList());

        assertThat(house.toString(), stringContainsInOrder(roomsAsStrings));
    }
}
