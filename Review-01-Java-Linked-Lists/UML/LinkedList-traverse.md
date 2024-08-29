# Basic Traversal Loop

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml traverse-00
hide empty members

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

object node_2 {
    data = 337
    next = null
}

ll::head -> node_0
ll::tail -> node_2

node_0::next -> node_1
node_1::next -> node_2

@enduml
```
</details>

![](traverse-00.svg)

<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml traverse-01
hide empty members

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

object node_2 {
    data = 337
    next = null
}

ll::head -> node_0
ll::tail -> node_2

node_0::next -> node_1
node_1::next -> node_2

object it {

}

it .[#0000FF].> node_0

@enduml
```
</details>

![](traverse-01.svg)


<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml traverse-02
hide empty members

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

object node_2 {
    data = 337
    next = null
}

ll::head -> node_0
ll::tail -> node_2

node_0::next -> node_1
node_1::next -> node_2

object it {

}

it .[#0000FF].> node_1

@enduml
```
</details>

![](traverse-02.svg)


<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml traverse-03
hide empty members

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

object node_2 {
    data = 337
    next = null
}

ll::head -> node_0
ll::tail -> node_2

node_0::next -> node_1
node_1::next -> node_2

object it {

}

it .[#0000FF].> node_2

@enduml
```
</details>

![](traverse-03.svg)


<details>
    <summary> PlantUML Object Diagram Source </summary>

```plantuml
@startuml traverse-04
hide empty members

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

object node_2 {
    data = 337
    next = null
}

ll::head -> node_0
ll::tail -> node_2

node_0::next -> node_1
node_1::next -> node_2

object it {

}

object null

it .[#0000FF].> null

@enduml
```
</details>

![](traverse-04.svg)
