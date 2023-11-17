import copy

import pytest
from hamcrest import *

from linkedlist import LinkedList
from typing import Any


from dataclasses import dataclass


@dataclass
class Datum:
    """
    This class exists to test the linked list __deepcopy__ due to how int and
    str "objects" are immutable
    """

    val: Any

    def __format__(self, spec) -> str:
        """
        This is an ugly hack to allow a Datum object to
        be included in an f-string that used an alignment, e.g, {:>4}
        """

        return str(self.val)


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


def test_constructor(get_empty_list):
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
    expected_strs = [
        f"Node # {idx:>4} - {datum:>4}" for idx, datum in enumerate(range(2, 4))
    ]
    assert_that(str(ll), string_contains_in_order(*expected_strs))


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

    with pytest.raises(StopIteration) as _err:
        next(it)


def test_str_after_append_various(build_interesting_list):
    all_src_data, ll = build_interesting_list

    expected_strs = [
        f"Node # {idx:>4} - {datum:>4}" for idx, datum in enumerate(all_src_data)
    ]
    assert_that(str(ll), string_contains_in_order(*expected_strs))


def test_deep_copy(build_interesting_list):
    all_src_data, ll_src = build_interesting_list

    ll_src.append(Datum(52))
    ll_src.append(Datum(42))
    ll_src.append(Datum(337))

    ll_copy = copy.deepcopy(ll_src)

    assert_that(ll_copy, is_(instance_of(LinkedList)))
    assert_that(ll_copy, has_length(len(ll_src)))

    for val_copy, val_src in zip(ll_copy, ll_src):
        assert_that(val_copy, is_(equal_to(val_src)))

        # Skip int and str entries... literals will always be the same object
        if any(isinstance(val_copy, a_type) for a_type in [int, str]):
            continue

        #  print(val_copy)
        assert_that(val_copy, is_(not_((same_instance(val_src)))))
