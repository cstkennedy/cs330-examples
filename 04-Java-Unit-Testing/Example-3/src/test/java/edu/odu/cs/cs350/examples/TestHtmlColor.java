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
        black  = new HtmlColor();
        white  = new HtmlColor(255, 255, 255);
        red    = new HtmlColor(255, 0, 0);
        green  = new HtmlColor(0, 255, 0);
        blue   = new HtmlColor(0, 0, 255);
        rColor = new HtmlColor(7, 62, 55);
    }

    @ParameterizedTest
    @ValueSource(ints = {-100, -5, -1, 0})
    public void testClampBelowRange(int value)
    {
        assertThat(HtmlColor.clamp(value), is(0));
    }

    @ParameterizedTest
    @ValueSource(ints = {0, 10, 100, 200, 250, 254, 255})
    public void testClampInRange(int value)
    {
        assertThat(HtmlColor.clamp(value), is(value));
    }

    @ParameterizedTest
    @ValueSource(ints = {256, 300, 700, 1000})
    public void testClampAboveRange(int value)
    {
        assertThat(HtmlColor.clamp(value), is(255));
    }

    @Test
    public void testDefaultConstructor()
    {
        HtmlColor color = new HtmlColor();

        assertThat(color.getRed(), is(0));
        assertThat(color.getBlue(), is(0));
        assertThat(color.getGreen(), is(0));

        assertThat(color.hashCode(), is(black.hashCode()));
        assertThat(color.toString(), equalTo("#000000"));
    }

    @Test
    public void testNonDefaultConstructor()
    {
        HtmlColor color = new HtmlColor(7, 62, 55);

        assertThat(color.getRed(), is(7));
        assertThat(color.getGreen(), is(62));
        assertThat(color.getBlue(), is(55));

        assertThat(rColor.hashCode(), is(color.hashCode()));

        String base16String = String.format(
            "#%02X%02X%02X",
            color.getRed(),
            color.getGreen(),
            color.getBlue()).toUpperCase();

        assertThat(color.toString(), equalTo(base16String));
    }

    @Test
    public void testNonDefaultConstructorInvalidRed()
    {
        HtmlColor color = new HtmlColor(-1, 62, 55);

        assertThat(color.getRed(), is(0));
        assertThat(color.getGreen(), is(62));
        assertThat(color.getBlue(), is(55));

        color = new HtmlColor(700, 62, 55);
        assertThat(color.getRed(), is(255));
        assertThat(color.getGreen(), is(62));
        assertThat(color.getBlue(), is(55));
    }

    @Test
    public void testNonDefaultConstructorInvalidGreen()
    {
        HtmlColor color = new HtmlColor(0, -1, 55);

        assertThat(color.getRed(), is(0));
        assertThat(color.getGreen(), is(0));
        assertThat(color.getBlue(), is(55));

        color = new HtmlColor(0, 620, 55);

        assertThat(color.getRed(), is(0));
        assertThat(color.getGreen(), is(255));
        assertThat(color.getBlue(), is(55));
    }

    @Test
    public void testNonDefaultConstructorInvalidBlue()
    {
        HtmlColor color = new HtmlColor(0, 0, -55);

        assertThat(color.getRed(), is(0));
        assertThat(color.getGreen(), is(0));
        assertThat(color.getBlue(), is(0));

        color = new HtmlColor(0, 62, 550);
        assertThat(color.getRed(), is(0));
        assertThat(color.getGreen(), is(62));
        assertThat(color.getBlue(), is(255));
    }

    @Test
    public void testSetRed()
    {
        HtmlColor color = (HtmlColor) black.clone();
        int oldHashCode = color.hashCode();

        color.setRed(100);

        assertThat(color.getRed(), is(100));
        assertThat(color.getBlue(), is(0));
        assertThat(color.getGreen(), is(0));

        assertThat(color, is(equalTo(color)));
        assertThat(color, is(not(equalTo(black))));
        assertThat(color.hashCode(), not(oldHashCode));
    }

    @Test
    public void testSetGreen()
    {
        HtmlColor color = (HtmlColor) blue.clone();
        int oldHashCode = color.hashCode();

        color.setGreen(100);

        assertThat(color.getRed(), is(0));
        assertThat(color.getBlue(), is(255));
        assertThat(color.getGreen(), is(100));

        assertThat(color, is(not(equalTo(blue))));
        assertThat(color.hashCode(), not(oldHashCode));
    }

    @Test
    public void testSetBlue()
    {
        HtmlColor color = (HtmlColor) white.clone();
        int oldHashCode = color.hashCode();

        color.setBlue(100);

        assertThat(color.getRed(), is(255));
        assertThat(color.getBlue(), is(100));
        assertThat(color.getGreen(), is(255));

        assertThat(color, is(not(equalTo(white))));
        assertThat(color.hashCode(), not(oldHashCode));
        assertThat(color.toString(), equalTo("#FFFF64"));

        //assertTrue(color.toString().contains("64"));
        assertThat(color.toString(), containsString("64"));
        assertThat(color.toString(), containsString("FFFF"));
    }
}
