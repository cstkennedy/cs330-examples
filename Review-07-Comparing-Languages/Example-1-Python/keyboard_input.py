"""
A basic keyboard Input Stream example.
"""


def main():
    """
    Python "main" function.
    """

    input_int = int(input("Enter an Integer: "))
    input_float = float(input("Enter a Double: "))
    input_char = input("Enter a Character: ")[0]
    input_boolean = bool(input("Enter a Boolean: "))
    input_string_no_spaces = input("Enter an String (no spaces): ")
    input_string_whole_line = input("Enter a String (with spaces): ")

    print()
    print("You Entered:")
    print(f"  Item  1: {input_int:}")
    print(f"  Item  2: {input_float:4.2f}")
    print(f"  Item  3: {input_char:}")
    print(f"  Item  4: {input_boolean:}")
    print(f"  Item  5: {input_string_no_spaces:}")
    print(f"  Item  6: {input_string_whole_line:}")


if __name__ == "__main__":
    main()
