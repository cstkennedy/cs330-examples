#include <string>
#include <iostream>
#include <iomanip>
#include <cstdlib>
#include <cmath>
#include <algorithm>
#include <iterator>
#include <vector>
#include <list>

using namespace std;

/**
 * Return a pair of iterators representing the beginning and end of a container
 *
 * @tparam C a type "container"  that provides iterators
 */
template<class C>
std::pair<typename C::iterator, typename C::iterator> into_iterators(C& container)
{
    return {container.begin(), container.end()};
}

/**
 * @todo write documentation
 */
template<typename Iterator>
void __output_group(const Iterator start, const Iterator stop)
{
    Iterator it = start;
    while (it != stop) {
        std::cout << std::setw(12) << *it << " ";
        ++it;
    }
}

/**
 * @todo write documentationZ
 *
 * @pre stop is no further than the end of a collection
 */
template<typename Iterator>
void output_in_groups(const Iterator start, const Iterator stop, const int group_size)
{
    Iterator it = start;
    while (it != stop) {
        const int num_entries = std::distance(it, stop);
        int spaces_to_move = group_size;
        if (num_entries < group_size) {
            spaces_to_move = num_entries;
        }

        Iterator group_stop = it;
        std::advance(group_stop, spaces_to_move);

        __output_group(it, group_stop);
        std::cout << "\n";

        it = group_stop;
    }
}

/**
 * @todo write documentation
 */
template<typename Collection>
void output_in_groups(Collection& collection, const int group_size)
{
    using It = typename Collection::iterator;

    // It start = collection.begin();
    // It end = collection.end();
    auto [start, end] = into_iterators(collection);
    output_in_groups(start, end, group_size);
}

int main(int argc, char** argv)
{
    vector<std::string> words = {
        "C++",
        "Java",
        "Python",
        "loop",
        "Lua",
        "Perl",
        "PHP",
        "Wikipedia",
        ".NET",
        "Rust",
        "Minecraft",
        "Persona",
        "Scratch",
        "HTML",
        "ZFlip 3",
        "RaspberryPi",
        "Steam Deck",
        "CSS",
        "JavaScript"
    };

    // std::sort(words.begin(), words.end());
    // std::vector<std::string>::iterator

    // auto start = words.begin();
    // auto end = words.end();

    /*
    auto [start, end] = into_iterators(words);
    // ++start;
    // ++start;
    // ++start;
    auto stop = start;

    std::advance(start, 3);
    std::advance(stop, 14);
    const int num_spaces_btwn = std::distance(start, stop);
    std::cout << num_spaces_btwn << "\n";

    std::sort(start, stop);

    for (const std::string& str : words) {
        std::cout << str << "\n";
    }
    */

    // auto [start, end] = into_iterators(words);

    const int num_per_group = atoi(argv[1]);
    output_in_groups(words, num_per_group);



    return 0;
}
