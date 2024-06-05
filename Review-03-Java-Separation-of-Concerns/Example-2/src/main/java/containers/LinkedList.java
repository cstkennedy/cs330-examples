package containers;

import java.util.Collection;
import java.util.Iterator;

public class LinkedList<T> implements Collection<T>, Iterable<T>, Cloneable {
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

    public boolean add(T toAdd)
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
     * Add multiple values.
     */
    public boolean addAll(Collection<? extends T> everythingToAdd)
    {
        for (T val : everythingToAdd) {
            if (!this.add(val)) {
                return false;
            }
        }
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

    public int size()
    {
        return this.currentSize;
    }

    public boolean isEmpty()
    {
        return this.size() == 0;
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

    //--------------------------------------------------------------------------
    // Everything below this this point is to "complete" the Java 11 Collection
    // interface. None of these operations are supported by our LinkedList and
    // will either:
    //   - throw an UnsupportedOperation Exception.
    //   - return false
    //   - return null
    //--------------------------------------------------------------------------

    public boolean retainAll(Collection<?> c)
    {
        throw new UnsupportedOperationException();
    }

    public boolean remove(Object obj)
    {
        return false;
    }

    public boolean removeAll(Collection<?> c)
    {
        throw new UnsupportedOperationException();
    }

    public boolean contains(Object obj)
    {
        for (T entry : this) {
            if (entry.equals(obj)) {
                return true;
            }
        }

        return false;
    }

    public boolean containsAll(Collection<?> c)
    {
        for (Object entry : c) {
            if (!this.contains(entry)) {
                return false;
            }
        }

        return true;
    }

    public <T> T[] toArray(T[] array)
    {
        return null;
    }

    public  Object[] toArray()
    {
        return null;
    }
}
