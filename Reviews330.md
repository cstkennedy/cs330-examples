Title: Reviews CS 330
Author: Thomas J. Kennedy
TOC: yes

%define <\ExampleZip> <zipFile> {[zipFile](./zipFile)}

This page will be updated throughout the semester as we cover topics of import.

**Everything on this page is subject to change over the next 16 weeks.** I will tweak existing materials and add additional materials throughout the semester.


# Review 1: Linked Lists

  During this Review, I discussed Linked Lists and addressed questions I received via email and during my office hours.

  * Example 1 was an implementation of Linked Lists similar to what one would see at the end of CS 250.
    * I briefly discussed command line arguments during this example.
    * The Linked List destructor was introduced in this example.

  * Example 2 was a similar implementation, but with the Node struct moved within the scope of
    the Inventory class.

  * Example 3 included a discussion of the Big-3.
    * This included a discussion of the LeakTracer utility.

  * Example 4 covered the Copy-and-Swap Idiom.
    * I started with a brief review of how to explore an unfamiliar code.
    * I discussed the `-fsanitize` compilation flag and using the compiler to detect
      memory leaks.


All source code is available in \ExampleZip{Review-01.zip}.

This Review is split into three (3) recordings:

  - [Part 1 (in which I discuss Examples 1 & 2)](https://youtu.be/WwHUr1N2z7A).
  - [Part 2 (in which I discuss Example 3)](https://youtu.be/zuD8Oc4v22c).
  - [Part 3 (in which I discuss Example 4)](https://youtu.be/sBKR0KOG3fc).


# Review 2

  I discussed two main themes:

  1. ADTs as a contract and creating a complete ADT interface. Refer to the
     [C++ class checklist](doc:implementingADTS)).
  2. The abstraction provided by defining such an interface. We explored this by using the STL
     container interface as a case study.
    - Examples 1 through 7 were the focus.
    - Example 8 served to motivate the need for, based on the data-structure, a formal
      iterator implementation. We will revisit the Big-3 in Review 03.

  All source code is available in \ExampleZip{Review-02.zip}. The recorded review is located [here](https://youtu.be/dWcPTtZl6Gs). _Note that this is the Fall 2017 recording._


---

# Updates in Progress

 **Everything on this page is subject to change over the next 16 weeks.** I will tweak existing materials and add additional materials throughout the semester.


# Review 3

  This set of examples takes the Linked List and Dynamic Array examples from Review 02 and incorporates the Big-3. The final example includes an alternative appraoch to the assignment operator (copy and swap).

  All source code is available in \ExampleZip{Review-03.zip}. The recorded review is located [here](https://youtu.be/hi7rGZ4gWlw). _Note that this is the Fall 2017 recording._


# Review 4

This is a multi-part design discussion:

  1. ADTs & Top-Down Design - This is a revisiting of the appraoch taken
    for the CS 250 semester project.
      - The source code is available in \ExampleZip{Review-04.zip}
      - **This is not true design**. It is **motivation for a more formal methodology.**

  2. The Java Implementation

  3. The Python 3 Implementation

  4. The proper way to [**plan**](doc:designDiscussion)

There are currently no recordings for this discussion.


# Review 5: Pointer Mechanics

  This set of examples reviews the mechanics of pointers. This is a review of value-type, reference-type, and pointer-type variables as discussed on the whiteboard periodically during lecture.

  All source code is available in \ExampleZip{Review-05-Pointer-Review.zip}.

  **I am preparing to record a lecture on this example set.**


# Review 6: Inheritance

  During this Review , we discussed the C++ mechanics used when implementing inheritance.

  * Example 1 focuses on:
    * Virtual Functions
    * Pure Virtual Functions
    * Dynamic Binding

  * Example 2 introduces the Factory Model.

  * Examples 3 through 6 add a
  `ShapeCollection` container and discuss extensions
  to the `Shape` and `ShapeFactory` interfaces.

  Part 1 (in which I discuss the Examples 1 & 2) is located [here](https://youtu.be/2-sp-W9urQA).
  Part 2 (in which I discuss the Examples 3 through 7) is located [here](https://youtu.be/CWBtYalF-io)

  All source code is available in \ExampleZip{Review-06-CPP-Shapes.zip}.


# Review 7: Switching to Java

  This Review Session was a discussion of 2 examples:

  * Example 1 - An example of reading keyboard input using the Java `Scanner` class.
  * Example 2 - An example in which the Fibonacci Sequence is computed. This example includes
    C++ and Java examples. Note the Python 3 example is included for comparison. You will not
    be tested on Python.
  * Example 3 - An example of a CS 250 level Objected Oriented Program in Java (instead of C++).

    - This includes a brief discussion of Gradle and JUnit as topics of interest.
    - Gradle and JUnit are CS 350 topics that are related to CS 330. You will
      not be graded on Gradle or JUnit in CS 330.
    - For the **truly curious**, you can access my CS 350 Review Materials [here](doc:Reviews350)

  Part 1 of the discussion (Examples 1 and 2) is available [here](https://youtu.be/IMGWyvyVxOE) and Part 2 (Example 3) is available [here](https://youtu.be/C8lobDIZUoY).

  All source code is available in \ExampleZip{Review-07-Java-Toys.zip}.


# Review 8: Java Inheritance

  In this Review we discussed a Java implementation of the shapes example (Review 6) This example set presents the same concepts. This serves both as a transition into Java, and a comparison of C++ and Java.

  **Note** these examples are not written in pure [idiomatic](http://www.merriam-webster.com/dictionary/idiomatic) Java. It is Java code written as a transition into Java from C++. Emphasis is placed
  on the mechanics of Java--as they differ from C++ and resemble C++. (Yes, this means **I have been sloppy with decorators**)

  Part 1 (in which I discuss the Examples 1 through 3) is located [here](https://youtu.be/taSLzzgsVTI).
  Part 2 (in which I discuss the Examples 4 through 5) is located [here](https://youtu.be/3NH6E3x8ijg)

  All source code is available in \ExampleZip{Review-08-Java-Shapes.zip}.


# Review 9: Java Gui & Thread Discussion

This Review Session was a discussion of GUIs using the javax.swing library. Topics included:

  - Prime Number Generation
  - Constructing a GUI
  - Try-Catch Blocks
  - Immediate Classes
  - A Basic Thread

Part 1 is located [here](https://youtu.be/SBIygPN7rik).
Part 2 is located [here](https://youtu.be/yBNufxSem4M).

All source code is available in \ExampleZip{Review-09-GuiThread.zip}.


# Review 10: Java Thread & ThreadPool Discussion

This was a discussion of Runnable objects in the context of Threads and Thread Pools.

The recording is located [here](https://youtu.be/b9K3COmd_W8). All source code is available in \ExampleZip{Review-10-Threads.zip}.


# Review 11 Python

This is a discussion of Python 3 in the context of OOP.

The recorded discussion is available [here](https://youtu.be/tXQgB07XTZI). All source code is available in \ExampleZip{Review-11-Python3-Shapes.zip}.


# Review 12 More Python!

This is a discussion of selected examples from previous reviews re-implemented in Python 3.

The recorded discussion is available [here](https://www.youtube.com/watch?v=dJrsAadeW2A). All source code is available in \ExampleZip{Review-12-Python3-Toys.zip}.

