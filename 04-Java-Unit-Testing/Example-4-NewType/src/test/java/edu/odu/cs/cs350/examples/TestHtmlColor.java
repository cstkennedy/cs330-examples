package edu.odu.cs.cs350.examples;

import org.junit.jupiter.api.TestMethodOrder;
import org.junit.jupiter.api.MethodOrderer;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import static org.junit.jupiter.api.Assertions.*;

import static org.hamcrest.MatcherAssert.assertThat;
import static org.hamcrest.Matchers.*;
import org.hamcrest.core.IsNull;

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
public class TestHtmlColor {
    private HtmlColor black;
    private HtmlColor white;
    private HtmlColor red;
    private HtmlColor green;
    private HtmlColor blue;
    private HtmlColor rColor; // "random" color

    @BeforeEach
    public void setUp()
    {
        black = new HtmlColor();
        white = new HtmlColor(
            HtmlColor.Component.from_clamped(255),
            HtmlColor.Component.from_clamped(255),
            HtmlColor.Component.from_clamped(255)
        );
        red = new HtmlColor(
            HtmlColor.Component.from_clamped(255),
            HtmlColor.Component.from_clamped(0),
            HtmlColor.Component.from_clamped(0)
        );
        green = new HtmlColor(
            HtmlColor.Component.from_clamped(0),
            HtmlColor.Component.from_clamped(255),
            HtmlColor.Component.from_clamped(0)
        );
        blue = new HtmlColor(
            HtmlColor.Component.from_clamped(0),
            HtmlColor.Component.from_clamped(0),
            HtmlColor.Component.from_clamped(255)
        );
        rColor = new HtmlColor(
            HtmlColor.Component.from_clamped(7),
            HtmlColor.Component.from_clamped(62),
            HtmlColor.Component.from_clamped(55)
        );
    }

    @ParameterizedTest
    @ValueSource(ints = {-100, -5, -1, 0})
    public void testClampBelowRange(int value)
    {
        assertThat(HtmlColor.Component.clamp(value), is(0));
    }

    @ParameterizedTest
    @ValueSource(ints = {0, 10, 100, 200, 250, 254, 255})
    public void testClampInRange(int value)
    {
        assertThat(HtmlColor.Component.clamp(value), is(value));
    }

    @ParameterizedTest
    @ValueSource(ints = {256, 300, 700, 1000})
    public void testClampAboveRange(int value)
    {
        assertThat(HtmlColor.Component.clamp(value), is(255));
    }

    @Test
    public void testComponentFromInvalid()
        throws HtmlColor.InvalidColorException
    {
        assertThrows(HtmlColor.InvalidColorException.class,
            ()-> {
                var comp = HtmlColor.Component.from(-1);
            }
        );

        assertThrows(HtmlColor.InvalidColorException.class,
            ()-> {
                var comp = HtmlColor.Component.from(700);
            }
        );
    }

    @ParameterizedTest
    @ValueSource(ints = {0, 10, 100, 200, 250, 254, 255})
    public void testComponentFromInRange(int value)
        throws HtmlColor.InvalidColorException
    {
        assertThat(HtmlColor.Component.from(value).value(), is(value));
    }

    @ParameterizedTest
    @ValueSource(ints = {-100, -5, -1, 0})
    public void testComponentFromClampedBelowRange(int value)
    {
        assertThat(HtmlColor.Component.from_clamped(value).value(), is(0));
    }

    @ParameterizedTest
    @ValueSource(ints = {0, 10, 100, 200, 250, 254, 255})
    public void testComponentFromClampedInRange(int value)
    {
        assertThat(HtmlColor.Component.from_clamped(value).value(), is(value));
    }

    @ParameterizedTest
    @ValueSource(ints = {256, 300, 700, 1000})
    public void testCompoentFromClampedAboveRange(int value)
    {
        assertThat(HtmlColor.Component.from_clamped(value).value(), is(255));
    }

    @Test
    public void testDefaultConstructor()
    {
        HtmlColor color = new HtmlColor();

        assertThat(color.getRed().value(), is(0));
        assertThat(color.getBlue().value(), is(0));
        assertThat(color.getGreen().value(), is(0));

        assertThat(color.hashCode(), is(black.hashCode()));
        assertThat(color.toString(), equalTo("#000000"));
    }

    @Test
    public void testNonDefaultConstructor()
    {
        HtmlColor color = new HtmlColor(
            HtmlColor.Component.from_clamped(7),
            HtmlColor.Component.from_clamped(62),
            HtmlColor.Component.from_clamped(55)
        );

        assertThat(color.getRed().value(), is(7));
        assertThat(color.getGreen().value(), is(62));
        assertThat(color.getBlue().value(), is(55));

        assertThat(rColor.hashCode(), is(color.hashCode()));

        String base16String = String.format(
            "#%02X%02X%02X",
            color.getRed().value(),
            color.getGreen().value(),
            color.getBlue().value()).toUpperCase();

        assertThat(color.toString(), equalTo(base16String));
    }

    @Test
    public void testSetRed()
    {
        HtmlColor color = (HtmlColor) black.clone();
        int oldHashCode = color.hashCode();

        color.setRed(HtmlColor.Component.from_clamped(100));

        assertThat(color.getRed().value(), is(100));
        assertThat(color.getBlue().value(), is(0));
        assertThat(color.getGreen().value(), is(0));

        assertThat(color, is(equalTo(color)));
        assertThat(color, is(not(equalTo(black))));
        assertThat(color.hashCode(), not(oldHashCode));
    }

    @Test
    public void testSetGreen()
    {
        HtmlColor color = (HtmlColor) blue.clone();
        int oldHashCode = color.hashCode();

        color.setGreen(HtmlColor.Component.from_clamped(100));

        assertThat(color.getRed().value(), is(0));
        assertThat(color.getBlue().value(), is(255));
        assertThat(color.getGreen().value(), is(100));

        assertThat(color, is(not(equalTo(blue))));
        assertThat(color.hashCode(), not(oldHashCode));
    }

    @Test
    public void testSetBlue()
    {
        HtmlColor color = (HtmlColor) white.clone();
        int oldHashCode = color.hashCode();

        color.setBlue(HtmlColor.Component.from_clamped(100));

        assertThat(color.getRed().value(), is(255));
        assertThat(color.getBlue().value(), is(100));
        assertThat(color.getGreen().value(), is(255));

        assertThat(color, is(not(equalTo(white))));
        assertThat(color.hashCode(), not(oldHashCode));
        assertThat(color.toString(), equalTo("#FFFF64"));

        //assertTrue(color.toString().contains("64"));
        assertThat(color.toString(), containsString("64"));
        assertThat(color.toString(), containsString("FFFF"));
    }
}
