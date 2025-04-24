from collections.abc import Iterable, Iterator


def find_name(collection: Iterable, thing_to_find: str) -> str:
    """
    Search for a name

    @param start where to begin the search
    @param end where to end the search
    @param thing_to_find name to locate

    @returns postion of a match or end if no match was found
    """

    for cur_thing in collection:
        # Look at the current name (*search_it) and
        # compare to the name to find (thing_to_find)
        if cur_thing == thing_to_find:
            return cur_thing

    return None


def main():
    print("--------Start Example--------")
    names = ["Thomas", "Jay", "Steve", "Janet", "Ravi"]

    print("---------While Loop Example----------")

    it = iter(names)

    try:
        while name := next(it):
            # Dereference
            # Node: it->data
            # Iterator: *it (* is called the dereference operator)
            print(f"  - {name}")
    except StopIteration as _err:
        pass

    print("--------For-each Loop Example--------")
    for n in names:
        print(f"  - {n}")

    #--------------------------------------------------------------------------
    # Add a name - append demo
    #--------------------------------------------------------------------------
    print("-------------Add a Name--------------")
    new_name = "Hill"
    names.append(new_name)

    for n in names:
        print(f"  - {n}")

    #--------------------------------------------------------------------------
    # Search for a name
    #--------------------------------------------------------------------------
    print("----------Search for a Name----------")
    found_it = find_name(names, "Andrey")

    # found_it is either a name (string) or None
    if not found_it:
        print("No match was found")
    else:
        print("Found a Match")

    if "Steve" in names:
        print("Found a Match")
    else:
        print("No match was found")


if __name__ == "__main__":
    main()
