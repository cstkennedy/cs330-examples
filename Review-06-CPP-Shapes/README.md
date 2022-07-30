These examples cover the basics of inheritance in C++, including mechanics,
methodologies, and design paradigms. This discussion is split into four parts:

\bSidebar

After recording parts 1 through 4, I made some *style* tweaks to the code for
Examples 1 through 8. These tweaks dealt with code style (e.g., keeping
lines under 80 characters, spaces around operators, and spaces before
brackets).

\eSidebar

> Updated Examples 3, 4, and 5

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

# New Tweaks & Examples


  - **Example 1**
    - Basics of Polymorphism: Virtual Functions, Pure Virtual Functions, &
      Dynamic Binding

  - **Example 2**
    - Factory Pattern and *Interfaces as a Contract*

  - **Example 3**
    - `ShapeCollection`, object lifetimes, & iterators

  - **Example 4** 
    - `operator>>`

  - **Example 5**
    - Shape `read` method

  - **Example 6** 
    - **Removed** & **Replaced** - Fixing the Shape classes with modern C++
      - `override` keyword
      - `constexpr`
      - ` = default`
      - ` = delete`

  - **Example 7**
    - Separation of Concerns & Interfaces

 - **Example 8**
   - Replacing `ShapeCollection` with `std::vector` and smart pointers; updating `ShapeFactory`.

 - **Example 9**
    - Modern C++ additions, including:
      -  tuple unpacking
      - `string_views`

