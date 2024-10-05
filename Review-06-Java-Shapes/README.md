# Overview

These examples present many of the same **Object-Oriented** concepts... in the
context of Java. This discussion includes some comparisons of C++ and Java.
Topics include:

  - virtual methods
  - pure virtual (i.e., abstract) methods
  - dynamic binding
  - subtyping
  - interfaces
  - `@Override` decorators
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

  - **Example 1**
    - Basics of Polymorphism
    - The recording is available at <https://youtu.be/rR4Ai2kHOR0> or <https://odumedia.mediaspace.kaltura.com/media/CS+330+-+Java+Shapes+-+Example+1/1_7v2tg7er>

  - **Example 2**
    - Factory Pattern and *Interfaces as a Contract*
    - The recording is available at <https://youtu.be/R3t0FecJfRA> or <https://odumedia.mediaspace.kaltura.com/media/CS+330+-+Java+Shapes+-+Example+2/1_ak6fq5nh>

  - **Example 3**
    - Subtyping (e.g., `List` vs `ArrayList`) and Iterators
    - The recording is available at <https://youtu.be/z6_FzDDBQGI> or <https://odumedia.mediaspace.kaltura.com/media/CS+330+-+Java+Shapes+-+Example+3/1_2njgkdxo>

  - **Example 4** 
    - Input (`BufferedReader` and `Scanner`), Shape `read` Method, and More
      Iterators
    - The recording is available at <https://youtu.be/FteoxbG99Ic> or <https://odumedia.mediaspace.kaltura.com/media/CS+330+-+Java+Shapes+-+Example+4/1_ixmlacm9>

  - **Example 5**
    - A *better* `ShapeFactory` implementation using a `LinkedHashMap<String, Shape>`
    - The recording is available at <https://youtu.be/jf9OK0EH9bc> or <https://odumedia.mediaspace.kaltura.com/media/CS+330+-+Java+Shapes+-+Example+5/1_zrh07pda>

  - **Example 6**
    - `Stream` API, Lambda Functions, fixing `Shape.name()`, and **replacing**
      `Shape.read`
    - The recording is available at <https://youtu.be/AxWPjXq2SF8> or <https://odumedia.mediaspace.kaltura.com/media/CS+330+-+Java+Shapes+-+Example+6/1_1fd2x6gj>

  - **Example 7**
    - Separation of Concerns & Interfaces
    - The recording is available at <https://youtu.be/SltynAF6UkY> or <https://odumedia.mediaspace.kaltura.com/media/CS+330+-+Java+Shapes+-+Example+7/1_e2eqfw4c>

  - **Example 8**
    - Generics and Trait Bounds in `ShapeIterator`, using `extends` with
      interfaces, converting an `Iterator` to a `Stream`.
    - The recording is available at <https://youtu.be/Yz2nyYUAJOY> or <https://odumedia.mediaspace.kaltura.com/media/CS+330+-+Java+Shapes+-+Example+8/1_ed5cdhpi>

  - **Example 9 - Part 1**
    - Remove Copy Constructors and rewrite `clone` methods
    - Refactor driver logic and move IO logic into dedicated `ShapeParser`
    - The recording is available at <https://youtu.be/wLwSxvf-KdY> or <https://odumedia.mediaspace.kaltura.com/media/CS+330+-+Java+Shapes+-+Example+9+%28Part+1%29/1_p3b0q0lo>

  - **Example 9 - Part 2**
    - Remove Copy Constructors and rewrite `clone` methods
    - Refactor driver logic and move IO logic into dedicated `ShapeParser`
    - The recording is available at <https://youtu.be/HwrTr5fvCj0> or <https://odumedia.mediaspace.kaltura.com/media/CS+330+-+Java+Shapes+-+Example+9+%28Part+2%29/1_lr5tjj7t>



# Running the Examples

The examples can be run through Gradle using `./gradlew run`. This will:

  1. compile the Java source code
  2. run the compiled code with all necessary command line arguments


# Reports & Project Website

Example 6 includes JBake as a Gradle plugin. Run `./gradlew bake` to generate
a static website with documentation and reports. Open `build/jbake/home.html`
in your web browser.

> If you are curious about JBake... [take a look at my CS 350
> lecture](https://github.com/cstkennedy/cs350-examples/tree/master/Gradle-2-Reports).

