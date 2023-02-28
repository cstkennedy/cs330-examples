import sys


def main():

    for idx, arg in enumerate(sys.argv):
        print(f"{idx:>3}: {arg:<}")


if __name__ == "__main__":
    main()
