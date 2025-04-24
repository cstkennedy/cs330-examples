#include <string>
#include <iostream>
#include <cstdlib>
#include <cmath>
#include <algorithm>
#include <iterator>
#include <vector>
#include <list>
#include <fstream>

using namespace std;

using std::cout;
using std::string;
using std::istream_iterator;
using std::ostream_iterator;


bool is_non_empty(const std::string& str)
{
    if (str.size() == 0) {
        return false;
    }

    const size_t first_non_space_idx = str.find_first_not_of(" \t\n\r");

    return first_non_space_idx != string::npos;
}


template<template<typename...>  class Collection>
Collection<string> read_lines(istream& ins)
{
    Collection<string> raw_words;

    string line;
    while (getline(ins, line)) {
        if (is_non_empty(line)) {
            raw_words.push_back(line);
        }
    }

    return raw_words;
}

template<typename Collection = std::vector<char>>
bool contains_chars(string word, Collection banned = {'.', '+'})
{
    for (const char banned_char : banned) {
        if (word.find(banned_char) != string::npos) {
            return true;
        }
    }
    return false;
}


int main(int argc, char** argv)
{
    if (argc < 2) {
        std::cout << "No filename was provided" << '\n';
        return 1;
    }
    const char* filename = argv[1];
    ifstream input_file(filename);
    vector<string> raw_words = read_lines<std::vector>(input_file);

    vector<string> filtered_words;
    copy_if(begin(raw_words), end(raw_words), back_inserter(filtered_words),
            [](const string& word) {
                return word.size() < 5;
            });
    remove_if(begin(filtered_words), end(filtered_words),
              [](const string& word) {
                  return contains_chars(word);
              });

    for (const string& word : filtered_words) {
        cout << word << "\n";
    }


    return 0;
}
