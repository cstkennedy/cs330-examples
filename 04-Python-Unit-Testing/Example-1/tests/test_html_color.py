"""
1 - Does this piece of code perform the operations
    it was designed to perform?

2 - Does this piece of code do something it was not
    designed to perform?

1 Test per mutator
"""

import pytest
from hamcrest import *

from html_color import HtmlColor

"""
    private HtmlColor black
    private HtmlColor white
    private HtmlColor red
    private HtmlColor green
    private HtmlColor blue
    private HtmlColor rColor // "random" color

    @BeforeEach
    public void setUp()
        throws HtmlColor.InvalidColorException
    {
        black  = HtmlColor()
        white  = HtmlColor(255, 255, 255)
        red    = HtmlColor(255, 0, 0)
        green  = HtmlColor(0, 255, 0)
        blue   = HtmlColor(0, 0, 255)
        rColor = HtmlColor(7, 62, 55)
    }
"""

@pytest.fixture
def black() -> HtmlColor:
    yield HtmlColor()


def test_default_constructor(black: HtmlColor):
    color = HtmlColor()

    assert_that(color.red, is_(0))
    assert_that(color.green, is_(0))
    assert_that(color.blue, is_(0))

    assert_that(hash(color), is_(hash(black)))
    assert_that(str(color), equal_to("#000000"))


def test_non_default_constructor(black: HtmlColor):
    color = HtmlColor(7, 62, 55)

    assert_that(color.red, is_(7))
    assert_that(color.green, is_(62))
    assert_that(color.blue, is_(55))

    assert_that(hash(color), is_not(hash(black)))
    assert_that(str(color), equal_to("#073E37"))

"""

    @Test
    public void testNonDefaultConstructorInvalidRed()
        throws HtmlColor.InvalidColorException
    {
        assertThrows(HtmlColor.InvalidColorException.class,
            ()-> {
                HtmlColor color = HtmlColor(-1, 62, 55)
            }
        )

        assertThrows(HtmlColor.InvalidColorException.class,
            ()-> {
                HtmlColor color = HtmlColor(700, 62, 55)
            }
        )
    }
"""

"""
    @Test
    public void testNonDefaultConstructorInvalidGreen()
        throws HtmlColor.InvalidColorException
    {
        assertThrows(HtmlColor.InvalidColorException.class,
            ()-> {
                HtmlColor color = HtmlColor(0, -1, 55)
            }
        )

        assertThrows(HtmlColor.InvalidColorException.class,
            ()-> {
                HtmlColor color = HtmlColor(0, 620, 55)
            }
        )
    }

    @Test
    public void testNonDefaultConstructorInvalidBlue()
        throws HtmlColor.InvalidColorException
    {
        assertThrows(HtmlColor.InvalidColorException.class,
            ()-> {
                HtmlColor color = HtmlColor(0, 0, -55)
            }
        )

        assertThrows(HtmlColor.InvalidColorException.class,
            ()-> {
                HtmlColor color = HtmlColor(0, 62, 550)
            }
        )
    }

    @Test
    public void testSetRed()
        throws HtmlColor.InvalidColorException
    {
        HtmlColor color = (HtmlColor) black.clone()
        int oldHashCode = color.hashCode()

        color.setRed(100)

        assert_that(color.red, is_(100))
        assert_that(color.blue, is_(0))
        assert_that(color.green, is_(0))

        assert_that(color, is_(equal_to(color)))
        assert_that(color, is_(not(equal_to(black))))
        assert_that(color.hashCode(), not(oldHashCode))
    }

    @Test
    public void testSetGreen()
        throws HtmlColor.InvalidColorException
    {
        HtmlColor color = (HtmlColor) blue.clone()
        int oldHashCode = color.hashCode()

        color.setGreen(100)

        assert_that(color.red, is_(0))
        assert_that(color.blue, is_(255))
        assert_that(color.green, is_(100))

        assert_that(color, is_(not(equal_to(blue))))
        assert_that(color.hashCode(), not(oldHashCode))
    }

    @Test
    public void testSetBlue()
        throws HtmlColor.InvalidColorException
    {
        HtmlColor color = (HtmlColor) white.clone()
        int oldHashCode = color.hashCode()

        color.setBlue(100)

        assert_that(color.red, is_(255))
        assert_that(color.blue, is_(100))
        assert_that(color.green, is_(255))

        assert_that(color, is_(not(equal_to(white))))
        assert_that(color.hashCode(), not(oldHashCode))
        assert_that(color.toString(), equal_to("#FFFF64"))

        //assertTrue(color.toString().contains("64"))
        assert_that(color.toString(), containsString("64"))
        /assert_that(color.toString(), containsString("FFFF"))
    }
}
"""
