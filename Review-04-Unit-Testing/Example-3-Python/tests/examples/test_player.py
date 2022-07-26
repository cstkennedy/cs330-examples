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
        self.tom = Player("Tom")
        self.a_cylon = Player()
        self.the_doctor = Player("The Doctor")

        self.tom.set_symbol('X')

    def test_player_default_constructor(self):

        self.assertTrue(Player.is_generic(self.a_cylon))

        assert_that(self.a_cylon.get_symbol(), equal_to('?'))

        assert_that(hash(self.a_cylon), is_not(hash(self.tom)))
        assert_that(self.a_cylon, is_not(equal_to(self.tom)))

        # Hand wave... These are not the cylons you are looking for.
        assert_that(self.a_cylon.is_human(), is_(True))
        assert_that(self.a_cylon.is_computer(), is_(False))

    def test_player_constructor(self):

        self.assertEqual("Tom", str(self.tom))
        assert_that(str(self.tom), equal_to("Tom"))

        assert_that(hash(self.tom), is_not(hash(self.the_doctor)))
        assert_that(self.tom, is_not(equal_to(self.the_doctor)))

        assert_that(self.tom.is_human(), is_(True))
        assert_that(self.tom.is_computer(), is_(False))

    def test_set_symbol(self):

        old_hash_code = hash(self.tom)

        assert_that(self.tom.get_symbol(), is_('X'))
        assert_that(hash(self.tom), is_(old_hash_code))

        self.tom.set_symbol('O')
        assert_that(self.tom.get_symbol(), is_('O'))
        assert_that(hash(self.tom), is_(old_hash_code))

    def test_set_name(self):

        old_hash_code = hash(self.the_doctor)

        assert_that(self.the_doctor.get_name(), is_("The Doctor"))
        assert_that(hash(self.the_doctor), is_(old_hash_code))

        self.the_doctor.set_name("David Tennant")
        assert_that(self.the_doctor.get_name(), is_("David Tennant"))
        assert_that(hash(self.the_doctor), is_not(old_hash_code))

        self.the_doctor.set_name("Mat Smith")
        assert_that(self.the_doctor.get_name(), is_("Mat Smith"))
        assert_that(hash(self.the_doctor), is_not(old_hash_code))

        self.the_doctor.set_name("Peter Capaldi")
        assert_that(self.the_doctor.get_name(), is_("Peter Capaldi"))
        assert_that(hash(self.the_doctor), is_not(old_hash_code))

        self.the_doctor.set_name("Jodie Whittaker")
        assert_that(self.the_doctor.get_name(), is_("Jodie Whittaker"))
        assert_that(hash(self.the_doctor), is_not(old_hash_code))

        # No clone function, can't test equals

    def test_clone(self):

        the_original = copy.deepcopy(self.the_doctor)

        assert_that(hash(self.the_doctor), equal_to(hash(the_original)))
        assert_that(self.the_doctor, equal_to(the_original))
        assert_that(self.the_doctor.get_symbol(),
                    equal_to(the_original.get_symbol()))

        the_original.set_name("William Hartnell")
        assert_that(hash(self.the_doctor),
                    is_not(equal_to(hash(the_original))))
        assert_that(self.the_doctor, is_not(equal_to(the_original)))

    @unittest.skip("can not test")
    def test_next_move(self):

        # Can not test due to hardcoded System.in use in Player.next_move
        pass
