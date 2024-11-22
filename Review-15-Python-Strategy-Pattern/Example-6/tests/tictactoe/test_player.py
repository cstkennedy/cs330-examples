import copy

import pytest
from hamcrest import assert_that, equal_to, has_string, is_, is_not

import tictactoe.player as player
from tictactoe.builders import PlayerBuilder
from tictactoe.player import Player

"""
1 - Does this piece of code perform the operations
    it was designed to perform?

2 - Does this piece of code do something it was not
    designed to perform?

1 Test per mutator
"""


@pytest.fixture
def create_players():
    tom = Player(name="Tom", strategy=None)

    a_cylon = Player(strategy=None)
    the_doctor = Player(name="The Doctor", strategy=None)

    yield tom, a_cylon, the_doctor


def test_player_default_constructor(create_players):
    tom, a_cylon, _ = create_players

    assert_that(player.is_generic(a_cylon))

    assert_that(hash(a_cylon), is_not(hash(tom)))
    assert_that(a_cylon, is_not(equal_to(tom)))
    assert_that(not player.is_generic(tom))

    # Hand wave... These are not the cylons you are looking for.
    assert_that(a_cylon.is_human(), is_(False))
    assert_that(a_cylon.is_computer(), is_(True))
    assert_that(player.is_generic(a_cylon))


def test_player_constructor(create_players):
    tom, _, the_doctor = create_players

    assert_that(tom, has_string("Tom"))

    assert_that(hash(tom), is_not(hash(the_doctor)))
    assert_that(tom, is_not(equal_to(the_doctor)))

    assert_that(tom.is_human(), is_(False))
    assert_that(tom.is_computer(), is_(True))

    assert_that(not player.is_generic(tom))


def test_set_name(create_players):
    _, _, the_doctor = create_players

    old_hash_code = hash(the_doctor)

    assert_that(the_doctor.name, is_("The Doctor"))
    assert_that(hash(the_doctor), is_(old_hash_code))

    the_doctor.name = "David Tennant"
    assert_that(the_doctor.name, is_("David Tennant"))
    assert_that(hash(the_doctor), is_not(old_hash_code))

    the_doctor.name = "Mat Smith"
    assert_that(the_doctor.name, is_("Mat Smith"))
    assert_that(hash(the_doctor), is_not(old_hash_code))

    the_doctor.name = "Peter Capaldi"
    assert_that(the_doctor.name, is_("Peter Capaldi"))
    assert_that(hash(the_doctor), is_not(old_hash_code))

    the_doctor.name = "Jodie Whittaker"
    assert_that(the_doctor.name, is_("Jodie Whittaker"))
    assert_that(hash(the_doctor), is_not(old_hash_code))

    assert_that(the_doctor, has_string(the_doctor.name))
    assert_that(the_doctor.is_human(), is_(False))
    assert_that(the_doctor.is_computer(), is_(True))

    assert_that(not player.is_generic(the_doctor))


def test_clone(create_players):
    _, _, the_doctor = create_players

    the_original = copy.deepcopy(the_doctor)

    assert_that(hash(the_doctor), equal_to(hash(the_original)))
    assert_that(the_doctor, equal_to(the_original))

    the_original.name = "William Hartnell"
    assert_that(hash(the_doctor), is_not(equal_to(hash(the_original))))
    assert_that(the_doctor, is_not(equal_to(the_original)))


def test_next_move():
    bot_9001 = (
        PlayerBuilder.builder()
        .with_name("Tic-Tac-Toe Bot 9001")
        .with_strategy(name="SetMoves", moves=list(range(1, 10)))
        .build()
    )

    for expected_move in range(1, 10):
        assert_that(bot_9001.next_move(), is_(expected_move))

    # We have exhausted all moves... make sure that Player.next_move
    # does **not** silence exceptions
    with pytest.raises(IndexError):
        bot_9001.next_move()
