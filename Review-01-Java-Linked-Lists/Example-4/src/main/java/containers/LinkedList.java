package containers;


public class LinkedList implements Cloneable
{
    /**
     * Node is an implementation detail. It does not need to be visible to the
     * outside world.
     */
    private class Node {
        public int  data;
        public Node next;

        Node()
        {
            this.data = 0;
            this.next = null;
        }

        Node(int d)
        {
            this.data = d;
            this.next = null;
        }
    }

    /**
     * This is a pointer to the head (first)
     * Node
     */
    private Node head;

    /**
     * This is a pointer to the tail (last)
     * Node
     */
    private Node tail;

    /**
     * Current size of the LinkedList--e.g.,
     * current (actual) number of rooms
     */
    int currentSize;

    /**
     * Set up an empty list with a size of zero and no Nodes.
     */
    public LinkedList()
    {
        this.head = null;
        this.tail = null;
        this.currentSize = 0;
    }

    /**
     * Store a new piece of data
     *
     * @param toAdd datum to store
     *
     * @return true if toAdd was stored (it always is)
     */
    public boolean add(int toAdd)
    {
        Node newNode = new Node(toAdd);

        // If adding the first Node
        if (head == null) {
            head        = newNode;
            tail        = newNode;
            currentSize = 1;

            // Why set newNode to null?
            newNode     = null;

            return true;
        }

        // Link the newNode to the end
        // of the existing list
        tail.next = newNode;

        // Update tail;
        tail = tail.next;
        // tail = newNode;

        // Update the size
        ++currentSize;

        return true;
    }

    /**
     * A "quick-and-dirty" clear operation.
     *
     * For this implementation, we will set
     *   - head = null
     *   - tail = null
     *   - currentSize = 0
     */
    public void clear()
    {
        this.head = null;
        this.tail = null;
        this.currentSize = 0;
    }

    /**
     * Get the number of entries in the list.
     */
    public int size()
    {
        return this.currentSize;
    }

    /**
     * Get whether the list contains zero entries.
     */
    public boolean isEmpty()
    {
        return this.size() == 0;
    }

    /**
     * Create an identical (deep) copy of this list.
     */
    public LinkedList clone()
    {
        LinkedList copy = new LinkedList();

        Node it = this.head;
        while (it != null) {
            copy.add(it.data);

            it = it.next;
        }

        return copy;
    }

    public String toString()
    {
        StringBuilder bldr = new StringBuilder();

        int index = 0;   // Used to output ids
        Node it  = this.head;

        while (it != null) {
            bldr.append(
                String.format("Node # %4d - %4d%n", index, it.data
            ));

            // increment index
            index++;

            it = it.next;
        }

        return bldr.toString();
    }
}
