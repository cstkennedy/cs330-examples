Title: Review Sessions
Author: Thomas Kennedy
TOC: yes

%define <\ExampleDir> <ex> {[here](../ReviewExampleCode)}
%define <\ExampleZip> <zipFile> {[zipFile](../ReviewExampleCode/zipFile)}


# Review Session 1: Linked Lists

  During this Review Session, I discussed Linked Lists and addressed questions I recieved via email and during my office hours.

  * Example 1 was an implementation of Linked Lists simliar to what one would see at the end of CS 250.
    * I briefly discussed command line arguments during this example.
    * The Linked List destructor was introduced in this example.

  * Example 2 was a similar implementation, but with the Node struct moved within the scope of
    the Inventory class.

  * Example 3 included a discussion of the Big-3. 
    * This included a discussion of the LeakTracer utility.

  I concluded the discussion with brief discussion of rsync to transfer files.

  The recording of the Review Session is available [here](https://youtu.be/Yo8EXvhx_dw).
  All source code is available in Review-01.zip under the [Example Code](../ReviewExampleCode) section of the course site.


# Review Session 2 [review2]

  During this Review Session, I discussed two main themes:

  1. ADTs as a contract and creating a complete ADT interface. Refer to the 
     [C++ class checklist](../../Public/implementingADTS)).
  2. The abstraction provided by defining such an interface. We explored this by using the STL
     container interface as a case study.
    - Examples 1 through 7 were the focus.
    - Example 8 served to motivate the need for the Big-3 and, based on the data-structure, a formal
      iterator implementation. This will be the focal point for Review 03.


  The recording of the Review Session is available [here](https://youtu.be/KyNKnNI53rI).
  All source code is available in Review-02.zip under the [Example Code](../ReviewExampleCode) section of the course site.

  The written materials (e.g., candidate lists) can be found [here](../Lecture-01)


# Review Session 3

  This set of examples takes the Linked List and Dynamic Array examples from Review 02 and incorporates the Big-3.

  **There is no corresponding recording.**

  All source code is available in Review-03.zip under the [Example Code](../ReviewExampleCode) section of the course site.


# Review Session 4: Assignment 1 Postmortem

  This Review Session was a Postmortem for Assignment 1. My solution is available on the Submission page and under the as Review-04.zip under the [Example Code](../ReviewExampleCode) section of the course site

  The recording of the Review Session is available [here](https://youtu.be/YYdsnny1KB4).


# Review Session 5: Pointer Mechanics

  This set of examples reviews the mechanics of pointers..

  **There is no corresponding recording.**

  All source code is available in \ExampleZip{Review-05-Pointer-Review.zip} under the [Example Code](../ReviewExampleCode) section of the course site.


# Review Session 6: Inheritance 

  During this Review Session, we discussed the C++ mechanics used when implementing inheritance. 

  Example 1 focuses on:

  * Virtual Functions
  * Pure Virtual Functions
  * Dynamic Binding

  Example 2 introduces the Factory Model. 

  Examples 3 through 6 add a 
  `ShapeCollection` container and discuss extensions 
  to the `Shape` and `ShapeFactory` interfaces.

  Part 1 of the review (Examples 1-2) is available [here](https://youtu.be/IOYXiufiI0o), and Part 2 (Examples 3-6) is available [here](https://youtu.be/pBZRJIgS3HI)

  All source code is available in \ExampleZip{Review-06-CPP-Shapes.zip} the [Example Code](../ReviewExampleCode) section of the course site.

# Assignment 3 Postmortem

  This Review Session was a Postmortem for Assignment 3. My solution is available on the Submission page and under the Example Code Section of the site.

  The recording of the Review Session is available [here](https://youtu.be/-wrLXiNYWSo).

  All source code is available in \ExampleZip{Review-Assignment-3.zip} the [Example Code](../ReviewExampleCode) section of the course site.


# Review 7: Java Inheritance

  In this Review Session we discussed a Java implementation of the shapes example--i.e., Example 10 from Review Session 5. This example--available as ReviewExample11.zip--presents the same
  concepts demonstrated by Example 10. This serves both as a transition into Java and a comparison of C++ and Java.

  **Note** that Example 11 is not written in [idiomatic](http://www.merriam-webster.com/dictionary/idiomatic) Java. It is Java code written as a transition into Java from C++. Emphasis is placed
  on the mechanics of Java--as they differ from C++ and resemble C++.

  The recording of the Review Session is available [here](https://youtu.be/sCF7HRAym7g).

  All source code is available in \ExampleZip{Review-07-Java-Shapes.zip} the [Example Code](../ReviewExampleCode) section of the course site.

# Review 8: Switching to Java

  This Review Session was a discussion of 2 examples:

  * Example 1 - An example of reading keyboard input using the Java `Scanner` class.
  * Example 2 - An example in which the Fibonacci Sequence is computed. This example includes
    C++ and Java examples. Note the Python 3 example is included for comparison. You will not 
    be tested on Python.

  All source code is available in \ExampleZip{Review-08-Java-Toys.zip} the [Example Code](../ReviewExampleCode) section of the course site.

  There is currently no recording for these examples.

%if _ignoreForNow

# Review Session 4: Assignment 3 Postmortem

  This Review Session was a Postmortem for Assignment 2. My solution is available on the Submission page and under the as Review-04.zip under the [Example Code](../ReviewExampleCode) section of the course site

  The recording of the Review Session is available [here](https://youtu.be/2VWw9dY_g7k).


# Review Session 5: Switching to Java

  This Review Session was a discussion of 2 examples:

  * Example 1 - An example of reading keyboard input using the Java `Scanner` class.
  * Example 2 - An example in which the Fibonacci Sequence is computed. This example includes
    both C++ and Java examples.

  The code is available as Review-05.zip under the [Example Code](../ReviewExampleCode) section of the course site.

  The recording of the Review Session is available [here](https://youtu.be/EOOKX6rDXhQ).


# Review Session 6: Java Shapes Example

  In this Review Session we discussed a Java implementation of the shapes example--i.e., the Example from Review Session 3. This serves both as a transition into Java and a comparison of C++ and Java.

  **Note** that Example 6 is not written in [idiomatic](http://www.merriam-webster.com/dictionary/idiomatic) Java. It is Java code written as a transition into Java from C++. Emphasis is placed
  on the mechanics of Java--as they compare to C++.

  The recording of the Review Session is available 
  [here](https://youtu.be/wCohDswgWDE). All source code is available in the [Example Code](../ReviewExampleCode) section of the course site.




  The second set of Review Sessions, [CS 333 (CS 150 & CS 250) Review](#cs333), is a collection of review lectures from CS 333 last semester, Fall 2015.
  If you would like to review this material inmore detail, you can access the [Fall 2015 CS 333 site](https://www.cs.odu.edu/~tkennedy/cs333/f15kennedy/Directory/outline/).
  Note that while you will have access to the
  CS 333 lectures, you will not be able to access the CS 333 Assignments.





# Orientation Session [cs330]
  
  This session served as an introduction to course policies and materials. 
  The recording is available [here](https://connect.odu.edu/p4b9c3oreo7/).

# Review Session 1 [review1]

  During this Review Session, I discussed two main themes:

  1. The review of mechanics learned in CS 150 & CS 250 (or CS 333).
  2. The transition from a procedural--i.e., step-by-step approach--to problem solving
     wherein ADTs and classes are an afterthought to an Object Oriented approach.

  The recording of the Review Session is available [here](https://youtu.be/AEHpgrtsnrg). The lecture slides are 
  available [here](../Lecture-01/). A zip file containing
  the examples from this discussion are available [here](https://www.cs.odu.edu/~tkennedy/cs330/@sem@-supplemental/Protected/)
  as [Review-01.zip](https://www.cs.odu.edu/~tkennedy/cs330/@sem@-supplemental/Protected/Review-01.zip).

# Review Session 2 [review2]

  During this Review Session, I discussed two main themes:

  1. ADTs as a contract and creating a complete ADT interface. Refer to the 
     [C++ class checklist](../../Public/implementingADTS)).
  2. The abstraction provided by defining such an interface. We explored this by using the STL
     interface as a case study.

  A zip file containing the examples from this discussion are available 
  \ExampleDir{} as \ExampleZip{Review-02.zip}.

  The recording of the Review Session is available [here](https://youtu.be/WdepgI7dkiE).

# Review Session 3 [review3]

  During this Review Session, I discussed a single theme: 
  [**The Big-3**](../../Public/big3).

  A zip file containing the examples from this discussion are available 
  \ExampleDir{} as \ExampleZip{Review-03.zip}.

  The recording of the Review Session is available [here](https://youtu.be/FZus35BnzRc).

# Review 4: Assignment 1 Postmortem [review4]

  This Review Session served as our Assignment 1 review. 
  The former half of this discussion focussed on the Assignment 1 Solution:

  1. How the ADTs interacted--i.e., ADTs as a contract.
  2. A Review of the Big-3.
  3. Approaches to the Schedule::add method.

  The latter half of this discussion (Examples 4 through 8) focused on how we can improve the
  Schedule ADT if we relax the constraints imposed within Assignment 1.

  A zip file containing the examples from this discussion are available 
  \ExampleDir{} as \ExampleZip{Review-04-Assignment-1-Postmortem.zip}.

  I divided the recorded discussion into two parts--corresponding with the former and latter halves. 
  The first half this lecture is available [here](https://youtu.be/FYACkXpLRh8).
  The second half this lecture is available [here](https://youtu.be/FY5C4ggWc0g).

# Review Session 5: Midterm Review [midtermReview]

  This Review Session was a preparation for the Midterm Exam.
  
  * The discussion materials page is available [here](../midtermReview/).
  * The recording is available [here](https://youtu.be/y2VjSrrTQA8).
%endif

%if _lastSemester

# Review Session 1: 23 January
  During this Review Session, we discussed Linked Lists

  * Example 1 was an implementation of Linked Lists using member functions. It is similar to code that may be discussed
    during the first week of CS 250.

  * Example 2 was a more refined implementation that made use of
    * Additional member functions
    
    * Operator overloading

    * Data-hiding--i.e., private member data.

  * Example 3 was implemented using an inner class, a Copy Construtor, a Default Contructor, and an Assignment Operator. 

  The recording of the Review Session is available [here](https://connect.odu.edu/p9fxs4nclz3/).
  All source code is available in the [Example Code](https://www.cs.odu.edu/~tkennedy/cs330/supplemental/Protected/) section of the course site.

# Review Session 2: 04 February

  During this Review Session, we discussed three examples:
    
  * Review Example 3: An implementation of a Point ADT. This example demonstrated operator overloading, constructors, and destructors.
    * This example should be used as a technical reference--i.e., a guide on how to implement specific operators and functions. 
    * It--Review Example 3--is based on a mathematical defintion of a point in a Cartesian Plane--i.e., XY Plane. The Point
      can be converted to a Polar Coordinate in the form $(r, \theta )$.
  * Review Example 4: An intial implementaion of a bookmark manager. 
    * This example demonstrates operator overloading--including type-cast operators.
    * This example demonstrates abstraction. The *Bookmarks* ADT provides a container for *Bookmark* instances.
  * Review Example 5: A refined implementaion of a bookmark manager.
    * The *Bookmarks* container in this example provides the same interface--with a different implementation.
    * _Abstraction_ is a powerful tool.

  The recording of the Review Session is available [here](https://connect.odu.edu/p31vxnbxx02/).
  All source code is available in the [Example Code](https://www.cs.odu.edu/~tkennedy/cs330/supplemental/Protected/) section of the course site.
 


# Review Session 3: 03 March

  This Review Session was a Postmortem for Assignment 1. My solution is available on the Submission page and under the Example Code Section of the site.

  The recording of the Review Session is available [here](https://connect.odu.edu/p1svanp9jh9/).
  All source code is available in the [Example Code](https://www.cs.odu.edu/~tkennedy/cs330/supplemental/Protected/) section of the course site.
 
  
# Review Session 4: Midterm Review

  This Review Session was a preparation for the Midterm Exam.
  
  * The discussion materials page is available [here](../midtermReview/).
  * The recording is available [here](https://connect.odu.edu/p41osewkbai/).

# Review Session 5: 31 March

  During this Review Session, we discussed the C++ mechanics used when implementing inheritance. This includes:

  * Virtual Functions
  * Pure Virtual Functions
  * Dynamic Binding

  This is not an exhaustive list. There were two examples discussed, Example 9, ReviewExample09.zip, and Example 10, ReviewExample10.zip. 

  Example 9 is a series of three examples--built on the running Bookmark example--that demonstrates pure virtual functions. Example 10 is a more comprehensive example
  that demonstrates virtual functions **and** pure virtual functions. Example 10 introduces the Factory Model.

  The recording of the Review Session is available [here](https://connect.odu.edu/p8oaffoofpg/).
  All source code is available in the [Example Code](https://www.cs.odu.edu/~tkennedy/cs330/supplemental/Protected/) section of the course site.
   
# Review Session 6: 02 April

  This Review was an orientation to Assignment 3. Watch the recording of Review Session 5 prior to watching this recording. **There is no corresponding
  Example Code for this Review Session.** 

  During this Review Session, we discussed Assignment 3--i.e., reviewed the assignment page. We then selected and wrote most of the Tool class. Part of this process
  involved commenting out portions of the ItemFactory class. This permitted us to focus on `Tool` in isolation. You will need to type the Tool class manually--i.e., 
  you must watch the recording and type each line that we discussed.

  The recording of the Review Session is available [here](https://connect.odu.edu/p3c602nd6yj/).


# Review Session 7: 16 April

  This Review Session was a Postmortem for Assignment 3. My solution is available on the Submission page and under the Example Code Section of the site.

  The recording of the Review Session is available [here](https://connect.odu.edu/p190ombr6yr/).
  All source code is available in the [Example Code](https://www.cs.odu.edu/~tkennedy/cs330/supplemental/Protected/) section of the course site.


# Review Session 8: 16 April

  In this Review Session we discussed a Java implementation of the shapes example--i.e., Example 10 from Review Session 5. This example--available as ReviewExample11.zip--presents the same
  concepts demonstrated by Example 10. This serves both as a transition into Java and a comparison of C++ and Java.

  **Note** that Example 11 is not written in [idiomatic](http://www.merriam-webster.com/dictionary/idiomatic) Java. It is Java code written as a transition into Java from C++. Emphasis is placed
  on the mechanics of Java--as they differ from C++ and resemble C++.

  The recording of the Review Session is available [here](https://connect.odu.edu/p2fjzuxgm2k/).
  All source code is available in the [Example Code](https://www.cs.odu.edu/~tkennedy/cs330/supplemental/Protected/) section of the course site.


# Review Session 9: 17 April

  This Review was an orientation to Assignment 5. Watch the recordings of Review Sessions 6 and 7 prior to watching this recording. **There is no corresponding
  Example Code for this Review Session.** 

  During this Review Session, I discussed Assignment 5--i.e., reviewed the assignment page. I demonstrated how to write the Consumable class--based on the Solution in Assignment 3. You will need to type the Consumable class manually--i.e., 
  you must watch the recording and type each line that we discussed.

  **Note** that this assignment is not written in [idiomatic](http://www.merriam-webster.com/dictionary/idiomatic) Java. It is Java code written as a transition into Java. Emphasis is placed
  on the mechanics of Java--as they differ from C++ and resemble C++.

  The recording of the Review Session is available [here](https://connect.odu.edu/p7kl7cjvzdj/).


# Review Session 10: 24 April

  This Review Session was a discussion of GUIs using the `javax.swing` library. Topics included:

  * Prime Number Generation
  * Constructing a GUI
  * Try-Catch Blocks
  * Immediate Classes

  **Note** that the threads example is included for your interest. You will see threads in detail in CS 471 Operating Systems.

  The recording of the Review Session is available [here](https://connect.odu.edu/p1x4fe1yxen/). 
  The code discussed in this Review Session is available as ReviewExample13.zip in the [Example Code](https://www.cs.odu.edu/~tkennedy/cs330/supplemental/Protected/) section of the course site.

# Review Session 11: Final Review

  This Review Session was a preparation for the Final Exam.
  
  * The discussion materials page is available [here](../finalReview/).
  * The recording is available [here](https://connect.odu.edu/p997bnzac4i/).

%endif

