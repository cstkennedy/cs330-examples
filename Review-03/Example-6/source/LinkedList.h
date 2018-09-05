#ifndef LL_H_DEFINED
#define LL_H_DEFINED

template <typename T>
class LinkedList {

    public:
        struct Node{
            T     data;
            Node* next;

            /**
             * Node Constructor
             */
            Node(T d)
            :data(d),
             next(nullptr)
            {
            }
        };

        /**
         * A standard C++ STL style iterator.
         * <p>
         * Recall the rules on Class naming and the STL.
         * <p>
         * Note I have relaxed my rule regarding definitions
         * within class declarations.
         * <p>
         * These are rudimentary iterators. There are a number
         * of additions needed before we can require this complete--e.g.,
         * operator-> and iterator traits. The latter is beyond the scope
         * of this course.
         */
        struct iterator{
            private:
                Node* pseudoPointer;

            public:
                iterator()
                    :pseudoPointer(nullptr)
                {
                }

                iterator(Node* node)
                    :pseudoPointer(node)
                {                    
                }

                T& operator*() const
                {
                    assert(pseudoPointer != nullptr);
                    return pseudoPointer->data;
                }

                iterator operator++(int v)
                {
                    iterator temp(this->pseudoPointer);
                    
                    this->pseudoPointer = pseudoPointer->next;

                    return temp;
                }

                iterator operator++()
                {
                    this->pseudoPointer = pseudoPointer->next;

                    return *this;
                }

                bool operator== (const iterator &rhs) const
                {
                    return this->pseudoPointer == rhs.pseudoPointer;
                }

                bool operator!= (const iterator &rhs) const
                {
                    return this->pseudoPointer != rhs.pseudoPointer;
                }
        };

        /**
         * A standard C++ STL style const_iterator.
         * <p>
         * Recall the rules on Class naming and the STL.
         */
        struct const_iterator{
            private:
                const Node* pseudoPointer;

            public:
                const_iterator()
                    :pseudoPointer(nullptr)
                {
                }

                const_iterator(const Node* node)
                    :pseudoPointer(node)
                {                    
                }

                const T& operator*()
                {
                    assert(pseudoPointer != nullptr);
                    return pseudoPointer->data;
                }

                const_iterator operator++(int v)
                {
                    const_iterator ctemp(this->pseudoPointer);
                    
                    this->pseudoPointer = pseudoPointer->next;

                    return ctemp;
                }

                const_iterator operator++()
                {
                    this->pseudoPointer = pseudoPointer->next;

                    return *this;
                }

                bool operator== (const const_iterator &rhs) const
                {
                    return this->pseudoPointer == rhs.pseudoPointer;
                }

                bool operator!= (const const_iterator &rhs) const
                {
                    return this->pseudoPointer != rhs.pseudoPointer;
                }
        };


    private:
        /**
         * This is a pointer to the head (first)
         * Node
         */
        Node*         head;

        /**
         * This is a pointer to the tail (last)
         * Node
         */
        Node*         tail;

        /**
         * Current size of the LinkedList--e.g.,
         * current (actual) number of rooms
         */
        int              currentSize;

    public:
        LinkedList()
            :head(nullptr),
             tail(nullptr),
             currentSize(0)
        {
        }


        LinkedList(const LinkedList& src)
            :head(nullptr),
             tail(nullptr),
             currentSize(0)
        {
            Node* it = src.head;

            while (it != nullptr) {
                this->push_back(it->data);

                it = it->next;
            }
        }

        ~LinkedList()
        {
            // Deallocate the Linked List
            Node* it = this->head;

            while (it != nullptr) {
                Node* prev = it;

                it = it->next;
                delete prev;
            }
            it = nullptr;

            this->head = nullptr;
            this->tail = nullptr;
            // End Linked List Deallocation
        }


        void push_back(T toAdd)
        {
            Node* newNode = new Node(toAdd);

            // If adding the first Node
            if (head == nullptr) {
                head        = newNode;
                tail        = newNode;
                currentSize = 1;

                // Why set newNode to null?
                newNode     = nullptr;

                return;
            }

            // Link the newNode to the end
            // of the exiting list
            tail->next = newNode;

            // Update tail;
            tail = tail->next;
            //tail = newNode;

            // Update the size
            currentSize++;
        }

        /**
         *
         */
        iterator begin()
        {
            return iterator(head);
        }

        /**
         *
         */
        const_iterator begin() const
        {
            return const_iterator(head);
        }

        /**
         *
         */
        iterator end()
        {
            return iterator(nullptr);
        }

        /**
         *
         */
        const_iterator end() const
        {
            return const_iterator(nullptr);
        }

        size_t size() const
        {
            return currentSize;
        }

        LinkedList<T>& operator=(LinkedList<T> rhs)
        {
            using std::swap;

            swap(*this, rhs);
        }

        /**
         * Swap the contents of two `LinkedList`s
         * <p>
         * I am using a friend function here and only here (under protest)
         * <p>
         * [Refer here](http://stackoverflow.com/questions/3279543/what-is-the-copy-and-swap-idiom)
         */
        friend 
        void swap(LinkedList<T>& lhs, LinkedList<T>& rhs)
        {
            using std::swap;

            swap(lhs.head, rhs.head);
            swap(lhs.tail, rhs.tail);
            swap(lhs.currentSize, rhs.currentSize);
        }

};

#endif