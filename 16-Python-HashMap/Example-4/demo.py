import pprint as pp
import sys

from didactic.hashmap import HashMap


def main() -> None:
    num_buckets = int(sys.argv[1])

    print()
    print(f"{num_buckets=}")

    color_map: HashMap[str, str] = HashMap(num_buckets=num_buckets)

    color_map["Thomas"] = "Blue"
    color_map["Jay"] = "All of Them at the Same Time"
    color_map["Jessica"] = "Purple"
    color_map["Angela"] = "Pink"

    print(f"    {color_map['Thomas']=}")
    print(f"    {color_map['Jay']=}")
    print(f"    {color_map['Jessica']=}")
    print(f"    {color_map['Angela']=}")

    print()

    for name, favourite_color in color_map.items():
        print(f"    {name=:16}{favourite_color=}")

    print()
    print(color_map)

    print("-" * 80)
    print("Keys, Values, & Iterators".center(80))
    print("-" * 80)

    for name in color_map.keys():
        print(f"  {name}")

    print()
    for name in color_map:
        print(f"  {name}")

    print()
    for color in color_map.values():
        print(f"  {color}")

if __name__ == "__main__":
    main()
