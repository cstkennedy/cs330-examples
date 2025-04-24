import pprint as pp

from didactic.hashmap import HashMap


def main() -> None:
    print("HashMap")

    color_map: dict[str, str] = {}
    color_map["Thomas"] = "Blue"
    color_map["Jay"] = "All of Them at the Same Time"
    color_map["Jessica"] = "Purple"
    color_map["Angela"] = "Pink"

    pp.pprint(color_map)

    names = list(color_map)

    print()
    print(names)
    print()

    for num_buckets in (4, 16, 64, 256):
        print(f"{num_buckets=}")
        for name in names:
            the_hash = hash(name)
            idx = the_hash % num_buckets

            print(f"    {name:12} -> {the_hash=:>24} ({idx=})")


    """
    color_map: HashMap[str, str] = HashMap()
    color_map["Thomas"] = "Blue"
    color_map["Jay"] = "All of Them at the Same Time"
    color_map["Jessica"] = "Purple"
    color_map["Angela"] = "Pink"
    """

if __name__ == "__main__":
    main()
