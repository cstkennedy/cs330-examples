## Unit Test - Java & JUnit

This is a quick look at Interface Completeness, Unit Test, and the
Mutator-Accessor Strategy... in Java.

This discussion is split into three (3) parts...

  - **Part 1 (Example 1)**

    This is a discussion of a basic unit test that makes use of the
    mutator-accessor strategy (with a few "oversights"). Exceptions are not
    handled.

  - **Part 2 (Example 2)**

    This discussion covers how to address the shortcomings identified in Part 1.
    How to check that expected exceptions are thrown (with `assertThrows`) is
    covered.

  - **Part 3 (Example 3)**

    This discussion covers how to eliminate the unecesary exceptions by using
    the notion of clamping numbers to a specified range (in our case... 0 to 255).

    Testing a static function and parameterized tests are covered.

