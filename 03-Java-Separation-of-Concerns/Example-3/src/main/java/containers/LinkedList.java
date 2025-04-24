package containers;

import java.util.Collection;
import java.util.Iterator;

/**
 * This is a custom LinkedList class used to demonstrate how to both
 * implement a Linked List and implement the Java Collection interface.
 *
 * @param <T> type of data to store
 */
@SuppressWarnings({
    "PMD.CloneThrowsCloneNotSupportedException",
    "PMD.FieldDeclarationsShouldBeAtStartOfClass",
    "PMD.ProperCloneImplementation",
    "PMD.ShortClassName",
    "PMD.TooManyMethods"
})
public class LinkedList<T> implements Collection<T>, Iterable<T>, Cloneable {

    @SuppressWarnings({
        "PMD.NullAssignment"
    })
    private class Node<T> {
        private final T  data;
        private Node<T>  next;

        public Node()
        {
            this.data = null;
            this.next = null;
        }

        public Node(final T datum)
        {
            this.data = datum;
            this.next = null;
        }
    }

    // NOTE: Removed <T> from LinkedListIterator declaration
    // We want to inherit the <T> from LinkedList, not define a new <T>.
    public class LinkedListIterator implements Iterator<T>
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

        @Override
        public boolean hasNext()
        {
            return this.currentFocus != null;
        }

        @Override
        public T next()
        {
            final T currentData = this.currentFocus.data;

            this.currentFocus = this.currentFocus.next;

            return currentData;
        }
    }

    /**
     * This is a pointer to the head (first) Node.
     */
    private Node<T> head;

    /**
     * This is a pointer to the tail (last) Node.
     */
    private Node<T> tail;

    /**
     * Current size of the LinkedList--e.g., current (actual) number of rooms.
     */
    private int currentSize;

    @SuppressWarnings({
        "PMD.NullAssignment"
    })
    public LinkedList()
    {
        this.head = null;
        this.tail = null;
        this.currentSize = 0;
    }

    @SuppressWarnings({
        "PMD.OnlyOneReturn"
    })
    @Override
    public boolean add(final T toAdd)
    {
        final Node<T> newNode = new Node<>(toAdd);

        // If adding the first Node
        if (this.head == null) {
            this.head        = newNode;
            this.tail        = newNode;
            this.currentSize = 1;

            return true;
        }

        // Link the newNode to the end of the existing list
        this.tail.next = newNode;
        this.tail = tail.next;

        ++currentSize;

        return true;
    }

    /**
     * Add multiple values.
     */
    @SuppressWarnings({
        "PMD.OnlyOneReturn"
    })
    @Override
    public boolean addAll(final Collection<? extends T> everythingToAdd)
    {
        for (final T val : everythingToAdd) {
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
    @SuppressWarnings({
        "PMD.NullAssignment"
    })
    @Override
    public void clear()
    {
        this.head = null;
        this.tail = null;
        this.currentSize = 0;
    }

    @Override
    public int size()
    {
        return this.currentSize;
    }

    @Override
    public boolean isEmpty()
    {
        return this.size() == 0;
    }

    @Override
    public Iterator<T> iterator()
    {
        return new LinkedListIterator();
    }

    @Override
    public LinkedList<T> clone()
    {
        final LinkedList<T> copy = new LinkedList<>();

        for (final T entry : this) {
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

    @Override
    public boolean retainAll(final Collection<?> collection)
    {
        throw new UnsupportedOperationException();
    }

    @Override
    public boolean remove(final Object obj)
    {
        return false;
    }

    @Override
    public boolean removeAll(final Collection<?> collection)
    {
        throw new UnsupportedOperationException();
    }

    @SuppressWarnings({
        "PMD.OnlyOneReturn"
    })
    @Override
    public boolean contains(final Object obj)
    {
        for (T entry : this) {
            if (entry.equals(obj)) {
                return true;
            }
        }

        return false;
    }

    @SuppressWarnings({
        "PMD.OnlyOneReturn"
    })
    @Override
    public boolean containsAll(final Collection<?> c)
    {
        for (Object entry : c) {
            if (!this.contains(entry)) {
                return false;
            }
        }

        return true;
    }

    @Override
    public <T> T[] toArray(final T[] array)
    {
        return null;
    }

    @Override
    public  Object[] toArray()
    {
        return null;
    }
}
