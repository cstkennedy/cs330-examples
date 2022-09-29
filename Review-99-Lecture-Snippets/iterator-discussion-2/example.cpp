#include <string>
#include <iostream>
#include <cstdlib>
#include <cmath>
#include <algorithm>
#include <iterator>
#include <vector>
#include <list>

using namespace std;


int main(int argc, char** argv)
{
    vector<std::string> raw_words = {
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

    std::vector<std::string> filtered_words;
    std::copy_if(begin(raw_words), end(raw_words), std::back_inserter(filtered_words),
                 [](const std::string& word) {
                     return word.size() > 3;
                 });

    std::sort(begin(filtered_words), end(filtered_words),
              // [](const auto& lhs, const auto& rhs) -> bool
              [](const std::string& lhs, const std::string& rhs) -> bool {
                  if (lhs.size() == rhs.size()) {
                      return lhs < rhs;
                  }
                  return lhs.size() < rhs.size();
              });

    // for (const std::string& word : filtered_words) {
        // cout << word << "\n";
    // }

    std::copy(begin(filtered_words), end(filtered_words),
              std::ostream_iterator<string>(cout, "\n"));


    return 0;
}
