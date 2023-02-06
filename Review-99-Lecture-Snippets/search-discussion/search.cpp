#include <vector>
#include <list>
#include <array>
#include <string>
#include <algorithm>
#include <iterator>
#include <utility>
#include <iostream>

#include "LinkedList.h"

namespace didactic_functions {
    //--------------------------------------------------------------------------
    template <typename T>
    int search(const T* anArr, const int length, const T& key)
    {
        for (int i = 0; i < length; ++i) {
            if (anArr[i] == key) {
                return i;
            }
        }

        return -1;
    }

    //--------------------------------------------------------------------------
    template <typename T>
    int search(const std::vector<T>& aVec, const T& key)
    {
        for (int i = 0; i < aVec.size(); ++i) {
            if (aVec[i] == key) {
                return i;
            }
        }

        return -1;
    }

    //--------------------------------------------------------------------------
    template <typename T>
    typename LinkedList<T>::Node* search(LinkedList<T> ll, const T& key)
    {
        typename LinkedList<T>::Node* it = ll.first_node();

        while (it != ll.last_node()) {
           if (it->data == key) {
                return it;
            }

           it = it->next;
        }

        return nullptr;
    }

    //--------------------------------------------------------------------------
    template <typename T>
    auto search(std::list<T> aList, const T& key)
    {
        typename std::list<T>::const_iterator it = aList.begin();

        while (it != aList.end()) {
           if (*it == key) {
                return it;
            }

           ++it;
        }

        return aList.end();
    }

    /**
     * Implementation of a "search function" based on `find` as documented at
     * https://en.cppreference.com/w/cpp/algorithm/find
     *
     * @tparam InputIt iterator for the source container
     *
     * @todo discuss why this is incomplete
     */
    template<class InputIt, typename T>
    InputIt search(InputIt itInputStart,
                    InputIt itInputEnd,
                    const T& key)
    {
        InputIt it = itInputStart;

        while (it != itInputEnd) {
            if (*it == key) {
                return it;
            }

            ++it;  // Pre-increment avoids a temporary copy.
        }

        return itInputEnd;
    }
}

/**
 * Demonstrate the use of "search" and iterators for a statically allocated
 * array, a vector, and a list.
 */
int main(const int argc, const char* const* argv)
{
    // Set up input data
    const std::array<std::string, 6> some_names =
        {{ "Thomas", "Jay", "Steve", "Janet", "Jim", "Mike" }};

    //--------------------------------------------------------------------------
    // Demo Array
    //--------------------------------------------------------------------------
    {
        std::string names[6];
        std::copy(begin(some_names), end(some_names), begin(names));

        const auto location_iterator = didactic_functions::search(begin(names), end(names), "Steve");
        const bool was_found = location_iterator != end(names);
        const unsigned int idx = location_iterator - begin(names);

        if (was_found) {
            std::cout << "Was Found... as entry # " << idx <<"!" << '\n';
        }
    }

    //--------------------------------------------------------------------------
    // Demo vector
    //--------------------------------------------------------------------------
    {
        std::vector<std::string> names(6);
        std::copy(begin(some_names), end(some_names), begin(names));

        const auto location_iterator = didactic_functions::search(begin(names), end(names), "Thomas");
        const bool was_found = location_iterator != end(names);
        const unsigned int idx = location_iterator - begin(names);

        if (was_found) {
            std::cout << "Was Found... as entry # " << idx <<"!" << '\n';
        }
    }

    //--------------------------------------------------------------------------
    // Demo list
    //--------------------------------------------------------------------------
    {
        std::list<std::string> names;
        std::copy(begin(some_names), end(some_names), std::back_inserter(names));

        const auto location_iterator = didactic_functions::search(begin(names), end(names), "Janet");
        const bool was_found = location_iterator != end(names);
        const unsigned int idx = std::distance(location_iterator, begin(names));

        if (was_found) {
            std::cout << "Was Found... as entry # " << idx <<"!" << '\n';
        }
    }

    return 0;
}
