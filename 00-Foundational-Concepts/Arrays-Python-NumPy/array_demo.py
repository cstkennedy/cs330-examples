import numpy as np

# Maximum allowed array size
MAX_SIZE: int = 10

# 20 Dash Divider
DIVIDER: str = "--------------------"


def main():
    static_array_demo()


def static_array_demo():
    """
    Statically allocated array demo - keep track of what is actually used
    """

    names = np.full(MAX_SIZE, "", dtype=np.dtype("U12"))
    used = 0

    names[used] = "Thomas"
    used += 1
    names[used] = "Jay"
    used += 1

    print(DIVIDER)
    for name in names[:used]:
        print(name)

    print(DIVIDER)


if __name__ == "__main__":
    main()
