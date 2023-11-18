import copy
from dataclasses import dataclass
from typing import Any

import pytest
from hamcrest import *

from linkedlist import LinkedList


@dataclass
class Datum:
    """
    This class exists to test the linked list __deepcopy__ due to how int and
    str "objects" are immutable
    """

    val: Any


@pytest.fixture()
def get_empty_list():
    yield LinkedList()


@pytest.fixture()
def build_interesting_list():
    ll = LinkedList()

    some_ints = list(range(1, 5))
    some_strings = ["Hello", "Python", "Hi"]

    data_to_add = some_ints + some_strings

    ll.extend(data_to_add)

    yield data_to_add, ll


def test_constructor_no_args(get_empty_list):
    empty_list = get_empty_list

    assert_that(empty_list.is_empty(), is_(True))
    assert_that(empty_list, is_(not_none()))
    assert_that(empty_list, has_length(0))

    # Classes with __str__ default to __repr__
    assert_that(empty_list, has_string("LinkedList()"))


def test_constructor_with_args():
    some_data = [Datum("Hello"), Datum(" "), Datum("Tom"), Datum("!" * 2)]

    ll = LinkedList(*some_data)
    assert_that(ll.is_empty(), is_(False))
    assert_that(ll, has_length(len(some_data)))

    assert_that(ll, contains_exactly(*some_data))


def test_append_int_once():
    ll = LinkedList()
    ll.append(1)

    assert_that(ll.is_empty(), is_(False))
    assert_that(ll, has_length(1))
    assert_that(ll, has_string("LinkedList(1)"))

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

    assert_that(ll.is_empty(), is_(False))
    assert_that(ll, has_length(2))

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

    # Check __str__
    expected_reprs = [
        repr(datum) for datum in range(2, 4)
    ]
    assert_that(repr(ll), string_contains_in_order(*expected_reprs))


def test_append_various(build_interesting_list):
    all_src_data, ll = build_interesting_list

    assert_that(ll, is_(instance_of(LinkedList)))

    assert_that(ll.is_empty(), is_(False))
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

    with pytest.raises(StopIteration) as _err:
        next(it)


def test_str_after_append_various(build_interesting_list):
    all_src_data, ll = build_interesting_list

    expected_reprs = [
        repr(datum) for datum in all_src_data
    ]
    assert_that(str(ll), string_contains_in_order(*expected_reprs))


def test_extend_int():
    ll = LinkedList()

    ll.append(-5)
    ll.append(-2)

    # Add the numbers 0, 2, 4, 6, 8, ..., 100
    some_ints = list(range(0, 101, 2))
    ll.extend(some_ints)

    all_data = [-5, -2] + some_ints

    assert_that(ll.is_empty(), is_(False))
    assert_that(ll, has_length(len(all_data)))

    assert_that(ll, contains_exactly(*all_data))


def test_deep_copy(build_interesting_list):
    all_src_data, ll_src = build_interesting_list

    ll_src.extend(Datum(val) for val in (52, 42, 337))

    ll_copy = copy.deepcopy(ll_src)

    assert_that(ll_copy, is_(instance_of(LinkedList)))

    assert_that(ll_copy.is_empty(), is_(False))
    assert_that(ll_copy, has_length(len(ll_src)))

    for val_copy, val_src in zip(ll_copy, ll_src):
        assert_that(val_copy, is_(equal_to(val_src)))

        # Skip int and str entries... literals will always be the same object
        if any(isinstance(val_copy, a_type) for a_type in [int, str]):
            continue

        #  print(val_copy)
        assert_that(val_copy, is_(not_((same_instance(val_src)))))

    expected_reprs = "LinkedList(" + ", ".join([repr(datum) for datum in ll_copy]) + ")"
    assert_that(ll_copy, has_string(expected_reprs))
