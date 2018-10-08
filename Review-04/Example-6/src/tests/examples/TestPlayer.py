from hamcrest import *
import unittest

from examples.player import Player



"""
 * 1 - Does this piece of code perform the operations 
 *     it was designed to perform?
 * 
 * 2 - Does this piece of code do something it was not 
 *     designed to perform?
 * 
 * 1 Test per mutator
"""

class TestPlayer(unittest.TestCase):

    def setUp(self):
        self.tom       = Player("Tom")
        self.aCylon    = Player()
        self.theDoctor = Player("The Doctor")

        self.tom.setSymbol('X')

    
    def testPlayerDefaultConstructor(self):

        self.assertTrue(Player.isGeneric(self.aCylon));

        assert_that(aCylon.getSymbol(), equal_to('?'));

        # assert_that(aCylon.hashCode(), is_not(tom.hashCode()));
        assert_that(aCylon, is_not(equal_to(tom)));
    

#     @Test
#     public void testPlayerConstructor()
#     {
#         assertEquals("Tom", tom.toString());
#         assert_that(tom.toString(), equal_to("Tom"));

#         assert_that(tom.hashCode(), is(not(theDoctor.hashCode())));
#         assert_that(tom, not(equal_to(theDoctor)));

#         assert_that(tom.isHuman(), is(true));
#         assert_that(tom.isComputer(), is(false));

#         // Hand wave... These are not the cylons you are looking for.
#         assert_that(aCylon.isHuman(), is(true));
#         assert_that(aCylon.isComputer(), is(false));
#     }

#     @Test
#     public void testSetSymbol()
#     {
#         int oldHashCode = tom.hashCode();

#         assert_that(tom.getSymbol(), is('X'));
#         assert_that(tom.hashCode(), is(oldHashCode));

#         tom.setSymbol('O');
#         assert_that(tom.getSymbol(), is('O'));
#         assert_that(tom.hashCode(), is(oldHashCode));
#     }

#     @Test
#     public void testSetName()
#     {
#         int oldHashCode = theDoctor.hashCode();

#         assert_that(theDoctor.getName(), is("The Doctor"));
#         assert_that(theDoctor.hashCode(), is(oldHashCode));

#         theDoctor.setName("David Tennant");
#         assert_that(theDoctor.getName(), is("David Tennant"));
#         assert_that(theDoctor.hashCode(), is(not(oldHashCode)));

#         theDoctor.setName("Mat Smith");
#         assert_that(theDoctor.getName(), is("Mat Smith"));
#         assert_that(theDoctor.hashCode(), is(not(oldHashCode)));

#         theDoctor.setName("Peter Capaldi");
#         assert_that(theDoctor.getName(), is("Peter Capaldi"));
#         assert_that(theDoctor.hashCode(), is(not(oldHashCode)));

#         theDoctor.setName("Jodie Whittaker");
#         assert_that(theDoctor.getName(), is("Jodie Whittaker"));
#         assert_that(theDoctor.hashCode(), is(not(oldHashCode)));

#         // No clone function, can't test equals
#     }

#     @Test
#     public void testClone()
#     {

#         Player theOriginal = theDoctor.clone();

#         assert_that(theDoctor.hashCode(), equal_to(theOriginal.hashCode()));
#         assert_that(theDoctor, equal_to(theOriginal));
#         assert_that(theDoctor.getSymbol(), equal_to(theOriginal.getSymbol()));

#         theOriginal.setName("William Hartnell");
#         assert_that(theDoctor.hashCode(), not(equal_to(theOriginal.hashCode())));
#         assert_that(theDoctor, not(equal_to(theOriginal)));
#     }

#     @Test
#     public void testNextMove()
#     {
#         // Can not test due to hardcoded System.in use in Player.nextMove
#         fail("Can not test");
#     }
# }