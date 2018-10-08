from hamcrest import *
import unittest

from examples.player import Player

import copy


class TestPlayer(unittest.TestCase):
    """
    1 - Does this piece of code perform the operations
        it was designed to perform?

    2 - Does this piece of code do something it was not
        designed to perform?

    1 Test per mutator
    """

    def setUp(self):
        self.tom       = Player("Tom")
        self.aCylon    = Player()
        self.theDoctor = Player("The Doctor")

        self.tom.setSymbol('X')

    def testPlayerDefaultConstructor(self):

        self.assertTrue(Player.isGeneric(self.aCylon))

        assert_that(self.aCylon.getSymbol(), equal_to('?'))

        assert_that(hash(self.aCylon), is_not(hash(self.tom)))
        assert_that(self.aCylon, is_not(equal_to(self.tom)))

    def testPlayerConstructor(self):

        self.assertEqual("Tom", str(self.tom))
        assert_that(str(self.tom), equal_to("Tom"))

        assert_that(hash(self.tom), is_not(hash(self.theDoctor)))
        assert_that(self.tom, is_not(equal_to(self.theDoctor)))

        assert_that(self.tom.isHuman(), is_(True))
        assert_that(self.tom.isComputer(), is_(False))

        # Hand wave... These are not the cylons you are looking for.
        assert_that(self.aCylon.isHuman(), is_(True))
        assert_that(self.aCylon.isComputer(), is_(False))

    def testSetSymbol(self):

        oldHashCode = hash(self.tom)

        assert_that(self.tom.getSymbol(), is_('X'))
        assert_that(hash(self.tom), is_(oldHashCode))

        self.tom.setSymbol('O')
        assert_that(self.tom.getSymbol(), is_('O'))
        assert_that(hash(self.tom), is_(oldHashCode))

    def testSetName(self):

        oldHashCode = hash(self.theDoctor)

        assert_that(self.theDoctor.getName(), is_("The Doctor"))
        assert_that(hash(self.theDoctor), is_(oldHashCode))

        self.theDoctor.setName("David Tennant")
        assert_that(self.theDoctor.getName(), is_("David Tennant"))
        assert_that(hash(self.theDoctor), is_not(oldHashCode))

        self.theDoctor.setName("Mat Smith")
        assert_that(self.theDoctor.getName(), is_("Mat Smith"))
        assert_that(hash(self.theDoctor), is_not(oldHashCode))

        self.theDoctor.setName("Peter Capaldi")
        assert_that(self.theDoctor.getName(), is_("Peter Capaldi"))
        assert_that(hash(self.theDoctor), is_not(oldHashCode))

        self.theDoctor.setName("Jodie Whittaker")
        assert_that(self.theDoctor.getName(), is_("Jodie Whittaker"))
        assert_that(hash(self.theDoctor), is_not(oldHashCode))

        # No clone function, can't test equals

    def testClone(self):

        theOriginal = copy.deepcopy(self.theDoctor)

        assert_that(hash(self.theDoctor), equal_to(hash(theOriginal)))
        assert_that(self.theDoctor, equal_to(theOriginal))
        assert_that(self.theDoctor.getSymbol(),
                    equal_to(theOriginal.getSymbol()))

        theOriginal.setName("William Hartnell")
        assert_that(hash(self.theDoctor), is_not(equal_to(hash(theOriginal))))
        assert_that(self.theDoctor, is_not(equal_to(theOriginal)))

    @unittest.skip("can not test")
    def testNextMove(self):

        # Can not test due to hardcoded System.in use in Player.nextMove
        pass
