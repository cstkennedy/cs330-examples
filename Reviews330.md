Title: Reviews CS 330
Author: Thomas J. Kennedy
TOC: yes

%define <\ExampleZip> <zipFile> {[zipFile](./zipFile)}

**Everything on this page is subject to change over the next 16 weeks.** I will
tweak existing materials and add additional materials throughout the semester.

---

If you are logged into one of the CS Linux servers you can copy the zip files
from

> `/home/tkennedy/Public/cs330`.

---

# Review 1: Linked Lists

During this Review, I discussed Linked Lists and addressed questions usually
received via email and during office hours.

\bSidebar

Note that while there is an Example 5, it will not be discussed in detail...
yet.

Example 6 will cover Memory Pools as a topic of interest. Just like Example 5,
you are not required (or expected) to apply this material on Assignment 1.

\eSidebar

  * **Example 1** was an implementation of Linked Lists similar to what one would
    see at the end of CS 250.
    * I briefly discussed command line arguments during this example.
    * The Linked List destructor was introduced in this example.
  * **Example 2** was a similar implementation, but with the Node struct moved
    within the scope of the Inventory class.
  * **Example 3** included a discussion of the Big-3.
    * This included a discussion of the LeakTracer utility.
  * **Example 4** covered the Copy-and-Swap Idiom.
    * I started with a brief review of how to explore an unfamiliar code.
    * I discussed the `-fsanitize` compilation flag and using the compiler to
      detect memory leaks.
  * **Example 5** is a preview of what we will have at the end of Review 03.
  * **Example 6** is a preview of what we will have at the end of Review 03.

All source code is available in \ExampleZip{Review-01-Linked-Lists.zip}. This Review is
split into four (4) recordings:

  - [Part 1 (in which I discuss Examples 1 & 2)](https://youtu.be/WwHUr1N2z7A).
  - [Part 2 (in which I discuss Example 3)](https://youtu.be/zuD8Oc4v22c).
  - [Part 3 (in which I discuss Example 4)](https://youtu.be/sBKR0KOG3fc).
  - [Part 4 (in which I cover Examples 5 & 6)](https://youtu.be/Ia8xUbjOei0)


All source code is available in [Review-01-Linked-Lists.zip](./Review-01-Linked-Lists.zip)


# Review 2: Well Defined Interfaces

I discussed two main themes in this review:

  1. ADTs as a contract and creating a complete ADT interface.
    - Read through [C++ class checklist](doc:implementingADTS).
    - **Skim** through [Abstraction with Iterators](doc:designDiscussionIterators).
  2. The abstraction provided by defining such an interface. We explored this
     by using the STL container interface as a case study.

Examples 1 through 7 were the focus. Example 8 served to motivate the need for,
based on the data-structure, a formal iterator implementation. We will revisit
the Big-3 in Review 03 (i.e., add iterators to memory-leak free codes).

The recorded review is located [here](https://youtu.be/dWcPTtZl6Gs). _Note that
this is the Fall 2017 recording._


All source code is available in [Review-02-Well-Defined-Interfaces.zip](./Review-02-Well-Defined-Interfaces.zip)


# Review 3: Separation of Concerns (And No Memory Leaks)

This discussion takes the iterator examples from Review 02 and incorporates the
Big-3 from Review 01 (to prevent memory leaks).

  - Examples 1 & 2 take the Linked List and Dynamic Array examples from Review
    02 and incorporate the Big-3, using the `Copy-and-Swap` implementation of
    the assignment operator.

  - Examples 3 through 5 cover how to generalize our code with: template
    tricks, and iterators.

  - Example 6 is *the fun part*.

The recorded review is split into three parts:

  1. [Part 1](https://youtu.be/inUskhq0SHo) - Examples 1 and 2.
  2. [Part 2](https://youtu.be/5-f_ej3cUJg) - Example 3 (review) through Example 5.
  2. [Part 3](https://youtu.be/s_1-4G8Gl4c) - Example 6.
  2. [Part 4](https://odu.zoom.us/rec/play/LZzOG7Yg89qTTBfkGhwQ3fgb5ndEhg3wk95KLkCzKELFXwEeEnZBjPYBFybpJiFhPbwS9XL1vj4LyASR.3uTf_KtFzWvm5QIQ) - Example 7 & Example 8 (Rust & Python Versions).


All source code is available in [Review-03-Separation-of-Concerns.zip](./Review-03-Separation-of-Concerns.zip)


# Review 4: Design, Tools, & Testing

In CS 250, you built a semester project. You went through a fairly contrived process:

  1. Write the Test Plan using [Black-Box Testing](https://www.cs.odu.edu/~zeil/cs333/latest/Public/bbtesting)
  2. Write Pseudocode through [Stepwise Refinement and Top-Down Design](https://www.cs.odu.edu/~zeil/cs333/latest/Public/stepwise/)
  3. Take that Pseudocode, take some ADTs, throw them together and hope for the best.
    - Your approach was not quite so [blasé](https://www.merriam-webster.com/dictionary/blas%C3%A9)
    - It was ineffective, unrefined, non-systematic, and backwards.
    - **We will start here...** after a whirlwind introduction to UML Class Diagrams.


## Whirlwind Introduction to UML Class Diagrams

One of the most difficult aspects of discussing object oriented code is
visualizing the pieces. To visualize classes and ADTs we can use UML Class
diagrams.

We will start by revisiting the C++ class checklist, mapping its parts into Java
and Python 3. We will then revisit Review 03 (Example 6) and build UML Class
diagrams with [PlantUML](http://plantuml.com/class-diagram).

The notes are available [here](doc:classChecklistsAndPlantUML).
The recorded discussion is available [here](https://youtu.be/GybR1wL_zzQ).


## Design: The CS 250 Way

ADTs & Top-Down Design - This is a revisiting of the approach taken for the CS
250 semester project.

 - The source code is available in \ExampleZip{Review-04.zip}
 - We will focus on **Examples 1 through 3**
 - **This is not true design**. It is **motivation for a more formal methodology.**

The recorded discussion is available [here](https://youtu.be/j2Y_Q3rVLsY).


## Proper Object Oriented Design and Analysis

The proper way to [**plan**](doc:designDiscussion) with objected oriented analysis.

The recorded discussion is available [here](https://youtu.be/sO86E30pxh0).


## Testing: C++, JUnit, Pyunit

This is a quick look at Interface Completeness and Unit Testing. We will look
at C++, Java, and Python 3 code. The source code is available in \ExampleZip{Review-04.zip}

The recorded review is split into two parts:

  1. [Part 1](https://youtu.be/8DQcCkq65kU) - C++ (Example 3) and Java (Examples 4-5).
  2. [Part 2](https://youtu.be/QPL927bYs7I) - Example 6 (Python 3).


All source code is available in [Review-04.zip](./Review-04.zip)


# Review 5: Pointer Mechanics

This set of examples reviews the mechanics of pointers. This is a review of
value-type, reference-type, and pointer-type variables as discussed on the
whiteboard periodically during lecture.

The recorded discussion is available [here](https://youtu.be/3BxfxTSO2U4).


All source code is available in [Review-05-Pointer-Mechanics.zip](./Review-05-Pointer-Mechanics.zip)


# Review 6: Inheritance in C++ (C++ Shapes)

These examples cover the basics of inheritance in C++, including mechanics,
methodologies, and design paradigms. This discussion is split into four parts:

\bSidebar

After recording parts 1 through 4, I made some *style* tweaks to the code for
Examples 1 through 8. These tweaks dealt with code style (e.g., keeping
lines under 80 characters, spaces around operators, and spaces before
brackets).

\eSidebar

  1. [Part 1](https://youtu.be/ZeM1OcxJcsA) (Example 1) focuses on:
    * Virtual Functions
    * Pure Virtual Functions
    * Dynamic Binding
  2. [Part 2](https://youtu.be/6GiGG2Kk7jw) (Examples 2 to 3) introduces the
     Factory Model and `ShapeCollection` container.
  3. [Part 3](https://youtu.be/-YSdI4FBUlo) (Examples 3 through 6) discusses
     extensions to the `Shape`, `ShapeFactory`, and `ShapeCollection`
     interfaces.
  4. [Part 4](https://youtu.be/E2SF6gmpG7Q) (Examples 7 through 8) revisits
     `ShapeFactory`, iterators, and lambda functions.
  5. [Part 5 (Zoom Lecture Recording)](https://odu.zoom.us/rec/share/zghIRrHbif183pvL8xRNfTzcjk8joqgmVKtxhp4i36T-NAQjDdBuVNmIF7G9W8w.bOq3D9iTwbLmxFv4) (Example 9)* covers a few modern C++
     additions, including:
    -  tuple unpacking
    - the `override` keyword
    - `string_views`
    - `constexpr`


All source code is available in [Review-06-CPP-Shapes.zip](./Review-06-CPP-Shapes.zip)


# Review 7: Switching to Java

This discussion is split into two parts:

  * Part 1 is available [here](https://youtu.be/IMGWyvyVxOE).
    * Example 1 - An example of reading keyboard input using the Java `Scanner`
      class.
    * Example 2 - An example in which the Fibonacci Sequence is computed. This
      example includes C++ and Java examples. Note the Python 3 example is
      included for comparison. You will not be tested on Python.
  * Part 2 is available [here](https://youtu.be/C8lobDIZUoY).
    * Example 3 - An example of a CS 250 level Objected Oriented Program in
      Java (instead of C++).
      - This includes a brief discussion of Gradle and JUnit as topics of
        interest.
      - Gradle and JUnit are CS 350 topics that are related to CS 330. You will
        not be graded on Gradle or JUnit in CS 330.
      - For the **truly curious**, you can access my CS 350 Review Materials
        [here](doc:Reviews350)


All source code is available in [Review-07-Switching-to-Java.zip](./Review-07-Switching-to-Java.zip)


# Review 8: Modeling Object Oriented Program Flow

Earlier this semester we discussed _UML Class Diagrams_ which we used to model
the things *(classes)* in a system. Now we need a method to capture flow (i.e.,
messages being passed) between *objects*.

## Whirlwind Introduction to UML Sequence Diagrams

We will start with a whirlwind introduction to UML Sequence diagrams in the
context of the [C++ Shapes Examples](#review-6-inheritance-in-c-c-shapes-)
(specifically Example 7 and Example 8).

The diagrams and markup to generate them is available in
\ExampleZip{Review-08-UML-Sequence-Diagrams.zip}

The recorded discussion is available [here](https://youtu.be/PusrbfgFfDI).


## Continuing Object Oriented Design and Analysis

The proper way to [**continue (validate & refine)**](doc:designDiscussionPart2)
an objected oriented domain model.

The recorded discussion is available [here](https://youtu.be/hlGoVWsb8Y4).


All source code is available in [Review-08-UML-Sequence-Diagrams.zip](./Review-08-UML-Sequence-Diagrams.zip)


# Review 9: Java Inheritance (Java Shapes)

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


All source code is available in [Review-09-Java-Shapes.zip](./Review-09-Java-Shapes.zip)


# Review 10: Java GUI & Thread Discussion

This is a discussion of GUIs using the javax.swing library.
Topics included:

  - Prime Number Generation
  - Constructing a GUI
  - Try-Catch Blocks
  - Immediate Classes
  - A Basic Thread

The recording is located [here](https://youtu.be/HAhLTNlXL3o).


All source code is available in [Review-10-GuiThread.zip](./Review-10-GuiThread.zip)


# Review 11: Java Thread & ThreadPool Discussion

This is a discussion of Runnable objects in the context of Threads and Thread
Pools.

The recording is located [here](https://youtu.be/2AJAWSDhqb4).


All source code is available in [Review-11-Threads.zip](./Review-11-Threads.zip)


# Review 12: Python Shapes

**This discussion will be updated after the live discussion during on 22 April
and 25 April.** I recommend you watch Reviews 13 and 14, then come back to this
one.

This is a discussion of Python 3 in the context of OOP.

The recorded **(Fall 2020)** discussion is available
[here](https://odu.zoom.us/rec/share/QDfa5OrYOTNQcG9BaJNv4m9dTd5GbHCQ1ZCkFuKVTHf0tyeFlLP1Vr6KnkR9gsa8.oBGZOIl4c0rn-4mG).


All source code is available in [Review-12-Python3-Shapes.zip](./Review-12-Python3-Shapes.zip)


# Review 13: More Python!

This is a discussion of selected examples from previous reviews re-implemented
in Python 3. We will briefly discuss:

  - Generator Expressions
  - Dictionary Comprehensions
  - Modules
  - `enumerate`
  - `pydoc`

The recorded discussion is available [here](https://youtu.be/i21FRLYxrZc).


All source code is available in [Review-13-Python3-Toys.zip](./Review-13-Python3-Toys.zip)


# Review 14: Python Linked List!

This is a revisiting of our first [example](../Review-01), but in Python! While
revisiting Linked Lists, in the context of Python 3, we will briefly discuss:

  - Iterators
  - Deep Copy
  - List Comprehensions
  - `@propery` and `@???.setter`
  - Name Mangling
  - `pydoc`

The recorded discussion is available [here](https://youtu.be/mtP36TTnuhs).


All source code is available in [Review-14-Python3-LinkedList.zip](./Review-14-Python3-LinkedList.zip)


# Review 99: Lecture Snippets

This is not a formal set of examples. Each piece of code comes from an informal
lecture discussion. 

  - `array-discussion` - brief review of array "resizing" from CS 250
  - `template-discussion` - brief overview/review of template functions  


All source code is available in [Review-99-Lecture-Snippets.zip](./Review-99-Lecture-Snippets.zip)
