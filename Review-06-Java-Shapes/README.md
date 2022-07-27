# Overview

This is set of Java Shape classes used as a case study for inheritance in Java.
Topics include:

These examples present many of the same **Object-Oriented** concepts... in the
context of Java. This discussion includes some comparisons of C++ and Java.
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

**Note** that the first few examples are not written in
[idiomatic](http://www.merriam-webster.com/dictionary/idiomatic) Java. By the
last example (i.e., Example 7)... we will have a "fairly good" codebase. *Note
that a few pieces of the Java class checklist (e.g., `equals`) are glossed
over.*

This discussion is split into seven (7) examples:

  1. [Part 1](https://youtu.be/_Qxt3yTuJOU) - Example 1 to Example 3
  2. [Part 2](https://youtu.be/PIPNWUCqwlk) - Example 4 to Example 5
  2. [Part 3](https://youtu.be/YE-Pu1ELoi4) - Example 6

  - **Example 1** - Basics of Polymorphism
  - **Example 2** - Factory Pattern and *Interfaces as a Contract*
  - **Example 3** - Subtyping (e.g., `List` vs `ArrayList`) and Iterators
  - **Example 4** - Input (`BufferedReader` and `Scanner`) and More Iterators
  - **Example 5** - Shape `read` method and a *better* `ShapeFactory` implementation
  - **Example 6** - `Stream` API, Lambda Functions, replacing `Shape.read`, and fixing `Shape.name`
  - **Example 7** - Separation of Concerns & Interfaces


# Running the Examples

The examples can be run through Gradle using `./gradlew run`. This will:

  1. compile the Java source code
  2. build a Jar file
  3. run the compiled code with all necessary command line arguments


# Reports & Project Website

Example 6 includes JBake as a Gradle plugin. Run `./gradlew bake` to generate
a static website with documentation and reports. Open `build/jbake/home.html`
in your web browser.

> If you are curious about JBake... [take a look at my CS 350
> lecture](https://github.com/cstkennedy/cs350-examples/tree/master/Gradle-2-Reports).

