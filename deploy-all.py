#! /usr/bin/env python

import sys
import os
import zipfile
import glob
import subprocess

from multiprocessing import Pool
from concurrent.futures import ProcessPoolExecutor


def prepare_example_set(example_set: str):
    """
    Prepare an Example set by running cleanup operations, then documentation
    generation.

    # Args
        example_set: directory containing one or more examples
    """

    for example_dir in glob.iglob(f"{example_set}/Example*"):
        if os.path.isfile(f"{example_dir}/makefile"):
            clean_col = "clean"
            example_type = "Makefile"

            try:
                subprocess.run(f"make -C {example_dir} clean".split(),
                               stdout=subprocess.DEVNULL,
                               stderr=subprocess.DEVNULL,
                               cwd=example_dir)

            except subprocess.CalledProcessError as err:
                clean_col = "Already clean"

            doc_col = "Generated"
            subprocess.run(f"make docs".split(),
                           stdout=subprocess.DEVNULL,
                           stderr=subprocess.DEVNULL,
                           cwd=example_dir)



        else:
            example_type = "-?-"
            clean_col = "-?-"
            doc_col = "-?-"

        print(f"|{example_dir:<50}|{example_type:^16}|{clean_col:^10}|{doc_col:^10}|")


def main():
    try:
        base_review_dir = sys.argv[1]

    except IndexError as err:
        base_review_dir = "."

    build_dir = f"{base_review_dir}/build"

    review_dirs = glob.glob(f"{base_review_dir}/Review-*")

    with ProcessPoolExecutor(max_workers=4) as executor:
        for review_dir in review_dirs:
            executor.submit(prepare_example_set, review_dir)

    for review_dir in review_dirs:
        zip_name = build_dir + "/" + review_dir.split("/")[-1] + ".zip"

        print(zip_name, "->", review_dir)

        with zipfile.ZipFile(zip_name, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as review_zip:
            for root_dir, _, files in os.walk(review_dir):
                for a_file in files:
                    review_zip.write(os.path.join(root_dir, a_file))


if __name__ == "__main__":
    main()


