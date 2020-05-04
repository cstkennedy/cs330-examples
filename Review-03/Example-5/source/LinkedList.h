#ifndef LL_H_DEFINED
#define LL_H_DEFINED

#include <utility>
#include <cassert>

template <typename T>
class LinkedList {
    private:
        struct Node {
            T     data;
            Node* next;

            Node(T d)
            :data(d),
             next(nullptr)
            {
            }
        };

    public:
        /**
         * A standard C++ STL style iterator (can be const or non-const).
         * <p>
         * Recall the rules on Class naming and the STL.
         * <p>
         * These is a rudimentary iterator. There are a number
         * of additions needed before we can claim this is complete--e.g.,
         * operator-> and iterator traits. The latter is beyond the scope
         * of this course.
         */
        template<bool is_const = true>
        class Iterator {
            public:
                using N  = typename std::conditional<is_const,
                                                     const Node, Node>::type;

                using CT = typename std::conditional<is_const,
                                                     const T, T>::type;
            private:
                N* pseudoPointer;

            public:
                Iterator()
                    :pseudoPointer(nullptr)
                {
                }

                Iterator(N* node)
                    :pseudoPointer(node)
                {
                }

                CT& operator*() const
                {
                    assert(pseudoPointer != nullptr);
                    return pseudoPointer->data;
                }

                Iterator operator++(int v)
                {
                    Iterator temp(this->pseudoPointer);

                    this->pseudoPointer = pseudoPointer->next;

                    return temp;
                }

                Iterator operator++()
                {
                    this->pseudoPointer = pseudoPointer->next;

                    return *this;
                }

                bool operator== (const Iterator &rhs) const
                {
                    return this->pseudoPointer == rhs.pseudoPointer;
                }

                bool operator!= (const Iterator &rhs) const
                {
                    return this->pseudoPointer != rhs.pseudoPointer;
                }
        };

        using iterator       = Iterator<false>;
        using const_iterator = Iterator<true>;

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
        int           currentSize;

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
            /*
            Node* it = src.head;

            while (it != nullptr) {
                this->push_back(it->data);
                it = it->next;
            }
            */
            for (const T& src_data : src) {
                this->push_back(src_data);
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
            // tail = newNode;

            // Update the size
            currentSize++;
        }

        iterator begin()
        {
            return iterator(head);
        }

        const_iterator begin() const
        {
            return const_iterator(head);
        }

        iterator end()
        {
            return iterator(nullptr);
        }

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

            return *this;
        }

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
