import sys

import numpy as np

D_LINE: str = "-" * 48
NUM_POWERS: int = 32


def main():
    # fmt: off
    # Naive Method
    # powers_of_two = np.zeros(NUM_POWERS, dtype=np.int32)

    # for i, _ in enumerate(powers_of_two):
    #     # Force conversion to np.int32 even if overflow occurs
    #     powers_of_two[i] = np.array(1 << i).astype(np.int32)
    # fmt: on

    # NumPy Broadcast Method
    powers_of_two = np.arange(0, NUM_POWERS, dtype=np.int32)
    powers_of_two = 2**powers_of_two

    print(D_LINE)
    for val in powers_of_two:
        print(f"{val:>15}")

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

    print(
        "size_of(np.int32):"
        + " " * 20
        + f"{size_in_bytes:>4} bytes / {size_in_bits:>4} bits"
    )

    size_in_bytes = sys.getsizeof(powers_of_two)
    size_in_bits = size_in_bytes << 3

    print(
        f"size_of(np.array({NUM_POWERS}, dtype=np.int32): "
        f"{size_in_bytes:>4} bytes / {size_in_bits:>4} bits"
    )


if __name__ == "__main__":
    main()
