import copy

import pytest
from hamcrest import *

from linkedlist import LinkedList


@pytest.fixture()
def get_empty_list():
    yield LinkedList()


@pytest.fixture()
def build_interesting_list():
    ll = LinkedList()

    some_ints = list(range(1, 5))
    some_strings = ["Hello", "Python", "Hi"]

    data_to_add = some_ints + some_strings

    for val in data_to_add:
        ll.append(val)

    yield data_to_add, ll


def test_empty_list(get_empty_list):
    empty_list = get_empty_list

    assert_that(empty_list, is_(not_none()))
    assert_that(empty_list, has_length(0))
    assert_that(empty_list, has_string(""))


def test_append_int_once():
    ll = LinkedList()
    ll.append(1)

    assert_that(ll, has_length(1))
    assert_that(ll, has_string("Node #    0 -    1"))

    it = iter(ll)
    val = next(it)

    assert_that(val, is_(instance_of(int)))
    assert_that(val, is_(equal_to(1)))

    # Check that we have exhausted the list
    with pytest.raises(StopIteration) as _err:
        next(it)


def test_append_int_twice():
    ll = LinkedList()
    ll.append(2)
    ll.append(3)

    assert_that(ll, has_length(2))
    assert_that(
        str(ll), string_contains_in_order("Node #    0 -    2", "Node #    1 -    3")
    )

    it = iter(ll)

    val = next(it)
    assert_that(val, is_(instance_of(int)))
    assert_that(val, is_(equal_to(2)))

    val = next(it)
    assert_that(val, is_(instance_of(int)))
    assert_that(val, is_(equal_to(3)))

    # Check that we have exhausted the list
    with pytest.raises(StopIteration) as _err:
        next(it)


def test_append_various(build_interesting_list):
    all_src_data, ll = build_interesting_list

    assert_that(ll, is_(instance_of(LinkedList)))
    assert_that(ll, has_length(len(all_src_data)))

    assert_that(ll, has_items(*all_src_data))
    assert_that(ll, contains_exactly(*all_src_data))

    # all_src_data and ll contain the same data in the same order
    it = zip(all_src_data, iter(ll))

    for src_val, ll_val in zip(all_src_data, iter(ll)):
        assert_that(ll_val, is_(equal_to(src_val)))

    # Confirm that the list does not contain any extra data
    it = iter(ll)

    for _ in ll:
        next(it)

    print(str(all_src_data))
    print(repr(all_src_data))
    with pytest.raises(StopIteration) as _err:
        next(it)


