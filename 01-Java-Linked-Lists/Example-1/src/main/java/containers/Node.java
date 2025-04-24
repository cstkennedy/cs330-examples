package containers;

/**
 * Node is an implementation detail. It should **not** be visible to the
 * outside world.
 * <p>
 * It should **not** be visible to the outside world. This includes classes
 * other than LinkedList
 */
 class Node {
    public int  data;
    public Node next;

    Node()
    {
        this.data = 0;
        this.next = null;
    }

    Node(int val)
    {
        this.data = val;
        this.next = null;
    }
}
