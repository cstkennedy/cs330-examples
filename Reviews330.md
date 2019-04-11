Title: Reviews CS 330
Author: Thomas J. Kennedy
TOC: yes

%define <\ExampleZip> <zipFile> {[zipFile](./zipFile)}

This page will be updated throughout the semester as we cover topics of import.

**Everything on this page is subject to change over the next 16 weeks.** I will
tweak existing materials and add additional materials throughout the semester.


# Review 1: Linked Lists

During this Review, I discussed Linked Lists and addressed questions I received
via email and during my office hours.

  * Example 1 was an implementation of Linked Lists similar to what one would
    see at the end of CS 250.
    * I briefly discussed command line arguments during this example.
    * The Linked List destructor was introduced in this example.

  * Example 2 was a similar implementation, but with the Node struct moved
    within the scope of the Inventory class.

  * Example 3 included a discussion of the Big-3.
    * This included a discussion of the LeakTracer utility.

  * Example 4 covered the Copy-and-Swap Idiom.
    * I started with a brief review of how to explore an unfamiliar code.
    * I discussed the `-fsanitize` compilation flag and using the compiler to
      detect memory leaks.


All source code is available in \ExampleZip{Review-01.zip}.

This Review is split into three (3) recordings:

  - [Part 1 (in which I discuss Examples 1 & 2)](https://youtu.be/WwHUr1N2z7A).
  - [Part 2 (in which I discuss Example 3)](https://youtu.be/zuD8Oc4v22c).
  - [Part 3 (in which I discuss Example 4)](https://youtu.be/sBKR0KOG3fc).


# Review 2

  I discussed two main themes:

  1. ADTs as a contract and creating a complete ADT interface. Refer to the
     [C++ class checklist](doc:implementingADTS)).
  2. The abstraction provided by defining such an interface. We explored this
     by using the STL container interface as a case study.
    - Examples 1 through 7 were the focus.
    - Example 8 served to motivate the need for, based on the data-structure, a
      formal iterator implementation. We will revisit the Big-3 in Review 03.

All source code is available in \ExampleZip{Review-02.zip}. The recorded review
is located [here](https://youtu.be/dWcPTtZl6Gs). _Note that this is the Fall
2017 recording._


# Review 3

This example takes the Linked List and Dynamic Array examples from
Review 02 and incorporates the Big-3 (Examples 1 to 4). Example 5 covers the
`Copy-and-Swap` implementation of the assignment operator.

Examples 6 through 8 cover how to generalize our code with: template tricks, and
iterators.

All source code is available in \ExampleZip{Review-03.zip}. The recorded
review is split into two parts:

  1. [Part 1](https://youtu.be/hi7rGZ4gWlw) - Examples 1 through 4.
  2. [Part 2](https://youtu.be/wmv6MgQSWpY) - Example 5 (review) through Example 8.


# Review 4: Design, Tools, and Testing

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


## Proper Object Oriented Design and Analysis

The proper way to [**plan**](doc:designDiscussion) with objected oriented analysis.

The recorded discussion is available [here](https://youtu.be/sO86E30pxh0).


## Design: The CS 250 Way

ADTs & Top-Down Design - This is a revisiting of the approach taken for the CS
250 semester project.

 - The source code is available in \ExampleZip{Review-04.zip}
 - We will focus on **Examples 1 through 3**
 - **This is not true design**. It is **motivation for a more formal methodology.**

The recorded discussion is available [here](https://youtu.be/j2Y_Q3rVLsY).


## Testing: C++, JUnit, Pyunit

This is a quick look at Interface Completeness and Unit Testing. We will look
at C++, Java, and Python 3 code. The source code is available in \ExampleZip{Review-04.zip}

The recorded review is split into two parts:

  1. [Part 1](https://youtu.be/8DQcCkq65kU) - C++ (Example 3) and Java (Examples 4-5).
  2. [Part 2](https://youtu.be/QPL927bYs7I) - Example 6 (Python 3).


# Review 5: Pointer Mechanics

This set of examples reviews the mechanics of pointers. This is a review of
value-type, reference-type, and pointer-type variables as discussed on the
whiteboard periodically during lecture.

  All source code is available in \ExampleZip{Review-05-Pointer-Review.zip}.

  The recorded discussion is available [here](https://youtu.be/3BxfxTSO2U4).


# Review 6: Inheritance in C++ (C++ Shapes)

  These examples cover the basics of inheritance in C++, including mechanics,
  methodologies, and design paradigms. This discussion is split into four parts:

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

  All source code is available in \ExampleZip{Review-06-CPP-Shapes.zip}.



# Review 7: Switching to Java

  This Review is split into two parts:

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

  All source code is available in \ExampleZip{Review-07-Java-Toys.zip}.


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

**A recording will be available after the live discussion (tentatively 09 April).**


## Continuing Object Oriented Design and Analysis

The proper way to [**continue (validate & refine)**](doc:designDiscussionPart2) an
objected oriented domain model.


**A recording will be available after the live discussion (tentatively 11 April).**


# Updates in Progress

**Everything on this page is subject to change over the next 16 weeks.** I will
tweak existing materials and add additional materials throughout the semester.

---

# Review 9: Java Inheritance

This a Java version of the
[C++ Shapes Discussion](#review-6-inheritance-in-c-c-shapes-) This example set
presents the many of the same concepts... in the context of Java. This
discussion continues our comparison of C++ and Java.

**Note** these examples are not written in
[idiomatic](http://www.merriam-webster.com/dictionary/idiomatic) Java. It is
Java code written as a transition into Java from C++. Emphasis is placed on the
mechanics of Java--as they parallel C++. (Yes, this means **I have been sloppy
with decorators**)


All source code is available in \ExampleZip{Review-09-Java-Shapes.zip}.


# Review 10: Java Gui & Thread Discussion

This Review Session was a discussion of GUIs using the javax.swing library.
Topics included:

  - Prime Number Generation
  - Constructing a GUI
  - Try-Catch Blocks
  - Immediate Classes
  - A Basic Thread

Part 1 is located [here](https://youtu.be/SBIygPN7rik).
Part 2 is located [here](https://youtu.be/yBNufxSem4M).

All source code is available in \ExampleZip{Review-09-GuiThread.zip}.


# Review 11: Java Thread & ThreadPool Discussion

This was a discussion of Runnable objects in the context of Threads and Thread
Pools.

The recording is located [here (Fall 2018)](https://youtu.be/b9K3COmd_W8). All
source code is available in \ExampleZip{Review-11-Threads.zip}.


# Review 12 Python

This is a discussion of Python 3 in the context of OOP.

The recorded discussion is available [here](https://youtu.be/tXQgB07XTZI). All
source code is available in \ExampleZip{Review-12-Python3-Shapes.zip}.


# Review 13 More Python!

This is a discussion of selected examples from previous reviews re-implemented
in Python 3.

The recorded discussion is available
[here](https://www.youtube.com/watch?v=dJrsAadeW2A). All source code is
available in \ExampleZip{Review-13-Python3-Toys.zip}.

# Review 14 Python Linked List!

This is a revisting of our first [example](#review-1), but in Python!








This is a discussion of selected examples from previous reviews re-implemented
in Python 3.

The recorded discussion is available
[here](https://www.youtube.com/watch?v=dJrsAadeW2A). All source code is
available in \ExampleZip{Review-12-Python3-Toys.zip}.
