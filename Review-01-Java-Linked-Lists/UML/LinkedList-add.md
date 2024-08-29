# Adding the First Node

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-01

object ll {
    head = null
    tail = null
    currentSize = 0
}

object newNode {
    data = 4
    next = null
}

@enduml
```
</details>

![](add-01.svg)

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-02

object ll {
    head
    tail = null
    currentSize = 0
}

object newNode {
    data = 4
    next = null
}

ll::head -> newNode

@enduml
```
</details>

![](add-02.svg)

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-03

object ll {
    head
    tail
    currentSize = 0
}

object newNode {
    data = 4
    next = null
}

ll::head -> newNode
ll::tail -> newNode

@enduml
```
</details>

![](add-03.svg)

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-04

object ll {
    head
    tail
    currentSize = 1
}

object newNode {
    data = 4
    next = null
}

ll::head -> newNode
ll::tail -> newNode

@enduml
```
</details>

![](add-04.svg)



# Adding the Second Node

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-05

object ll {
    head
    tail
    currentSize = 1
}

object node_0 {
    data = 4
    next = null
}

ll::head -> node_0
ll::tail -> node_0

@enduml
```
</details>

![](add-05.svg)

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-06

object ll {
    head
    tail
    currentSize = 1
}

object node_0 {
    data = 4
    next = null
}

object newNode {
    data = 9001
    next = null
}

ll::head -> node_0
ll::tail -> node_0

@enduml
```
</details>

![](add-06.svg)

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-07

object ll {
    head
    tail
    currentSize = 1
}

object node_0 {
    data = 4
    next
}

object newNode {
    data = 9001
    next = null
}

ll::head -> node_0
ll::tail -> node_0

node_0::next -> newNode

@enduml
```
</details>

![](add-07.svg)

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-08

object ll {
    head
    tail
    currentSize = 1
}

object node_0 {
    data = 4
    next
}

object newNode {
    data = 9001
    next = null
}

ll::head -> node_0
ll::tail -> newNode

node_0::next -> newNode

@enduml
```
</details>

![](add-08.svg)

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-09

object ll {
    head
    tail
    currentSize = 2
}

object node_0 {
    data = 4
    next
}

object newNode {
    data = 9001
    next = null
}

ll::head -> node_0
ll::tail -> newNode

node_0::next -> newNode

@enduml
```
</details>

![](add-09.svg)


# Adding the Third Node

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-10

object ll {
    head
    tail
    currentSize = 2
}

object node_0 {
    data = 4
    next
}

object node_1 {
    data = 9001
    next = null
}

object newNode {
    data = 337
    next = null
}

ll::head -> node_0
ll::tail -> node_1

node_0::next -> node_1

@enduml
```
</details>

![](add-10.svg)

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-11

object ll {
    head
    tail
    currentSize = 2
}

object node_0 {
    data = 4
    next
}

object node_1 {
    data = 9001
    next
}

object newNode {
    data = 337
    next = null
}

ll::head -> node_0
ll::tail -> node_1

node_0::next -> node_1
node_1::next -> newNode

@enduml
```
</details>

![](add-11.svg)


<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-12

object ll {
    head
    tail
    currentSize = 2
}

object node_0 {
    data = 4
    next
}

object node_1 {
    data = 9001
    next
}

object newNode {
    data = 337
    next = null
}

ll::head -> node_0
ll::tail -> newNode

node_0::next -> node_1
node_1::next -> newNode

@enduml
```
</details>

![](add-12.svg)

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml add-13

object ll {
    head
    tail
    currentSize = 3
}

object node_0 {
    data = 4
    next
}

object node_1 {
    data = 9001
    next
}

object newNode {
    data = 337
    next = null
}

ll::head -> node_0
ll::tail -> newNode

node_0::next -> node_1
node_1::next -> newNode

@enduml
```
</details>

![](add-13.svg)
