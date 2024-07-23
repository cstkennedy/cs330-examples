import pytest
from hamcrest import *


def test_name_not_set():
    pytest.fail("Test not yet written")


def test_name_too_short():
    pytest.fail("Test not yet written")


def test_flooring_type_not_set():
    pytest.fail("Test not yet written")


def test_flooring_type_too_short():
    pytest.fail("Test not yet written")


def test_flooring_cost_not_set():
    pytest.fail("Test not yet written")


def test_length_not_set():
    pytest.fail("Test not yet written")


def test_width_not_set():
    pytest.skip("This should never happen if length is set")


def test_flooring_cost_is_zero():
    pytest.fail("Test not yet written")


def test_flooring_cost_is_negative():
    pytest.fail("Test not yet written")


def test_length_is_zero():
    pytest.fail("Test not yet written")


def test_length_is_negative():
    pytest.fail("Test not yet written")


def test_width_is_zero():
    pytest.fail("Test not yet written")


def test_width_is_negative():
    pytest.fail("Test not yet written")


def test_build_success():
    pytest.fail("Test not yet written")
