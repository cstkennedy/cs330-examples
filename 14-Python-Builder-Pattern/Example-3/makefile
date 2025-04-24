PYTHON=python3.11

.PHONY: tests

tests:
	${PYTHON} -m pytest tests -v

coverage:
	${PYTHON} -m coverage run --branch -m pytest -v tests

	${PYTHON} -m coverage report

	${PYTHON} -m coverage html

test_fmt:
	${PYTHON} -m isort tests
	${PYTHON} -m black tests

test_typecheck:
	${PYTHON} -m mypy tests


