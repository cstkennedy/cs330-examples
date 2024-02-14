import sys

import numpy as np

D_LINE = "-" * 48


def main():
    num_powers = 32

    # Naive Method
    powers_of_two = np.zeros(num_powers, dtype=np.int32)

    print(D_LINE)

    # We should really use enumerate here
    for i in range(0, num_powers):
        # Force conversion to np.int32 even if overflow occurs
        powers_of_two[i] = np.array(1 << i).astype(np.int32)

    # NumPy Broadcast Method
    powers_of_two = np.arange(0, num_powers, dtype=np.int32)
    powers_of_two = 2**powers_of_two

    for val in powers_of_two:
        print(f"{val:}")

    print(f"\n{D_LINE}\n")

    for idx, val in enumerate(powers_of_two):
        # We get the address of the base array each time
        # This is due to NumPy-C bindings & optimizations
        array_address = val.__array_interface__["data"][0]
        print(f"{idx:>2}: {val:>11} {array_address:#x}")

    print(f"\n{D_LINE}\n")

    # Includes Garbage Collection overhead
    size_in_bytes = sys.getsizeof(np.int32(1))
    size_in_bits = size_in_bytes << 3

    print()
    print(f"size_of(np.int32): {size_in_bytes} bytes / {size_in_bits} bits")

    size_in_bytes = sys.getsizeof(powers_of_two)
    size_in_bits = size_in_bytes << 3

    print(
        "\n"
        f"size_of(np.array({num_powers}, dtype=np.int32): "
        f"{size_in_bytes} bytes / {size_in_bits} bits"
    )


if __name__ == "__main__":
    main()
