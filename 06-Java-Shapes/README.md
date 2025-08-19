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
last example (i.e., Example 9)... we will have a "fairly good" codebase. *Note
that a few pieces of the Java class checklist (e.g., `equals`) are glossed
over.*

This discussion is split into nine (9) examples:

  - **Example 1**
    - Basics of Polymorphism
    - The recording is available on
      [YouTube](https://youtu.be/rR4Ai2kHOR0)
      or
      [Panopto](https://odu.hosted.panopto.com/Panopto/Pages/Viewer.aspx?id=f274fe7d-fe60-48b7-ba6c-b3020118a5d8)

  - **Example 2**
    - Factory Pattern and *Interfaces as a Contract*
    - The recording is available on
      [YouTube](https://youtu.be/R3t0FecJfRA)
      or
      [Panopto](https://odu.hosted.panopto.com/Panopto/Pages/Viewer.aspx?id=0b99309c-604b-4071-97ab-b3020118e26f)

  - **Example 3**
    - Subtyping (e.g., `List` vs `ArrayList`) and Iterators
    - The recording is available on
      [YouTube](https://youtu.be/z6_FzDDBQGI)
      or
      [Panopto](https://odu.hosted.panopto.com/Panopto/Pages/Viewer.aspx?id=245f2f5f-d236-42f6-b525-b3020118fb2e)

  - **Example 4** 
    - Input (`BufferedReader` and `Scanner`), Shape `read` Method, and More
      Iterators
    - The recording is available on
      [YouTube](https://youtu.be/FteoxbG99Ic)
      or
      [Panopto](https://odu.hosted.panopto.com/Panopto/Pages/Viewer.aspx?id=cc6d9732-0ab5-4aaa-b38f-b30201191697)

  - **Example 5**
    - A *better* `ShapeFactory` implementation using a `LinkedHashMap<String, Shape>`
    - The recording is available on
      [YouTube](https://youtu.be/jf9OK0EH9bc)
      or
      [Panopto](https://odu.hosted.panopto.com/Panopto/Pages/Viewer.aspx?id=76ffae13-17d5-436d-a911-b30201194402)

  - **Example 6**
    - `Stream` API, Lambda Functions, fixing `Shape.name()`, and **replacing**
      `Shape.read`
    - The recording is available on
      [YouTube](https://youtu.be/AxWPjXq2SF8)
      or
      [Panopto](https://odu.hosted.panopto.com/Panopto/Pages/Viewer.aspx?id=c8162003-ddca-42e7-bea5-b30201196423)

  - **Example 7**
    - Separation of Concerns & Interfaces
    - The recording is available on
      [YouTube](https://youtu.be/SltynAF6UkY)
      or
      [Panopto](https://odu.hosted.panopto.com/Panopto/Pages/Viewer.aspx?id=110cc53d-526e-4807-845a-b30201197cf9)

  - **Example 8**
    - Generics and Trait Bounds in `ShapeIterator`, using `extends` with
      interfaces, converting an `Iterator` to a `Stream`.
    - The recording is available on
      [YouTube](https://youtu.be/Yz2nyYUAJOY)
      or
      [Panopto](https://odu.hosted.panopto.com/Panopto/Pages/Viewer.aspx?id=8ad06ff6-22ab-4468-91df-b3020119a41d)

  - **Example 9 - Part 1**
    - Remove Copy Constructors and rewrite `clone` methods
    - Refactor driver logic and move IO logic into dedicated `ShapeParser`
    - The recording is available on
      [YouTube](https://youtu.be/wLwSxvf-KdY)
      or
      [Panopto](https://odu.hosted.panopto.com/Panopto/Pages/Viewer.aspx?id=c533cace-131a-4897-abf7-b3020119ce5a)

  - **Example 9 - Part 2**
    - Remove Copy Constructors and rewrite `clone` methods
    - Refactor driver logic and move IO logic into dedicated `ShapeParser`
    - The recording is available on
      [YouTube](https://youtu.be/HwrTr5fvCj0)
      or
      [Panopto](https://odu.hosted.panopto.com/Panopto/Pages/Viewer.aspx?id=68fb0e16-ce1d-4891-9f50-b302011a172f)



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

