This a Java version of the
[C++ Shapes Discussion](#review-6-inheritance-in-c-c-shapes-). These examples
present many of the same concepts... in the context of Java. This discussion
continues our comparison of C++ and Java.

**Note** these examples are not written in
[idiomatic](http://www.merriam-webster.com/dictionary/idiomatic) Java. It is
Java code written as a transition into Java from C++. Emphasis is placed on the
mechanics of Java--as they parallel C++. (Yes, this means **I have been sloppy
with decorators**)

\bSidebar

If you are curious about JBake... [take a look at my CS 350
lecture](https://www.cs.odu.edu/~tkennedy/cs350/latest/Public/codeDocumentationJBake/index.html).

\eSidebar

This discussion is split into two parts:

  1. [Part 1](https://youtu.be/_Qxt3yTuJOU) - Example 1 to Example 3
  2. [Part 2](https://youtu.be/PIPNWUCqwlk) - Example 4 to Example 5
  2. [Part 3](https://youtu.be/YE-Pu1ELoi4) - Example 6

  - **Example 1**
  - **Example 2**
  - **Example 3**
  - **Example 4**
  - **Example 5**
  - **Example 6**
  - **Example 7**

---

# Overview

This is set of Java Shape classes used as a case study for inheritance in Java.
Topics include:

  - virtual methods
  - pure virtual (i.e., abstract) methods
  - dynamic binding
  - subtyping
  - interfaces
  - `@Override decorators`
  - lambda functions
  - Java's Stream API
  - Iterators
  - `S.O.L.I.D`


# Running the Examples

The examples can be run through Gradle using `./gradlew run`. This will:

  1. compile the Java source code
  2. build a Jar file
  3. run the compiled code with all necessary command line arguments

## Reports & Project Website

Example 6 includes JBake as a Gradle plugin. Run `./gradlew bake` to generate
a static website with documentation and reports. Open `build/jbake/home.html`
in your web browser.
