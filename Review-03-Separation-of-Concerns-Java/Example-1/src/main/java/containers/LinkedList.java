package containers;

import java.util.Iterator;

public class LinkedList<T> implements Iterable<T>, Cloneable {
    private class Node<T> {
        T     data;
        Node  next;

        Node()
        {
            this.data = null;
            this.next = null;
        }

        Node(T d)
        {
            this.data = d;
            this.next = null;
        }
    }

    public class LinkedListIterator<T> implements Iterator<T>
    {
        /**
         * This is used to keep track of the iterator's position. It is a
         * reference to the current Node.
         */
        private Node<T> currentFocus;

        /**
         * Create an iterator that is set the the first Node in the list.
         */
        public LinkedListIterator()
        {
            this.currentFocus = LinkedList.this.head;
        }

        public boolean hasNext()
        {
            return this.currentFocus != null;
                // && this.currentFocus.next != null;
        }

        public T next()
        {
            T currentData = this.currentFocus.data;

            this.currentFocus = this.currentFocus.next;

            return currentData;
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

    public LinkedList()
    {
        this.head = null;
        this.tail = null;
        this.currentSize = 0;
    }

    public void add(T toAdd)
    {
        Node newNode = new Node(toAdd);

        // If adding the first Node
        if (head == null) {
            head        = newNode;
            tail        = newNode;
            currentSize = 1;

            // Why set newNode to null?
            newNode     = null;

            return;
        }

        // Link the newNode to the end
        // of the existing list
        tail.next = newNode;

        // Update tail;
        tail = tail.next;
        // tail = newNode;

        // Update the size
        ++currentSize;
    }

    public int size()
    {
        return this.currentSize;
    }

    public Iterator<T> iterator()
    {
        return new LinkedListIterator();
    }

    public LinkedList<T> clone()
    {
        LinkedList<T> copy = new LinkedList<>();

        for (T entry : this) {
            copy.add(entry);
        }

        return copy;
    }
}
