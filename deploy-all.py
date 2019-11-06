#! /usr/bin/env python

import os
import zipfile
import glob
import subprocess
import argparse

from concurrent.futures import ProcessPoolExecutor
from typing import List


def prepare_example_set(example_set: str):
    """
    Prepare an Example set by running cleanup operations, then documentation
    generation.

    # Args
        example_set: directory containing one or more examples
    """

    for example_dir in glob.iglob(f"{example_set}/Example*"):
        prepare_example(example_dir)


def prepare_example(example_dir: str):
    """
    Prepare an Example directory by running cleanup operations, then
    documentation generation.

    # Args
        example_set: directory containing one or more examples
    """

    example_type = "-?-"
    clean_col = "-?-"
    doc_col = "-?-"

    if os.path.isfile(f"{example_dir}/gradlew"):
        clean_col = "clean"
        example_type = "Gradle"

        try:
            subprocess.run(f"./gradlew --no-daemon clean".split(),
                           stdout=subprocess.DEVNULL,
                           stderr=subprocess.DEVNULL,
                           cwd=example_dir)

        except subprocess.CalledProcessError as err:
            clean_col = "Already clean"

        doc_col = "Javadoc"
        subprocess.run(f"./gradlew --no-daemon  javadoc".split(),
                       stdout=subprocess.DEVNULL,
                       stderr=subprocess.DEVNULL,
                       cwd=example_dir)

    if os.path.isfile(f"{example_dir}/Cargo.toml"):
        clean_col = "clean"
        example_type = "Cargo"

        try:
            subprocess.run(f"cargo clean".split(),
                           stdout=subprocess.DEVNULL,
                           stderr=subprocess.DEVNULL,
                           cwd=example_dir)

        except subprocess.CalledProcessError as err:
            clean_col = "Already clean"

        doc_col = "Rustdoc"
        subprocess.run(f"cargo rustdoc".split(),
                       stdout=subprocess.DEVNULL,
                       stderr=subprocess.DEVNULL,
                       cwd=example_dir)

    elif os.path.isfile(f"{example_dir}/makefile"):
        clean_col = "clean"
        example_type = "Makefile"

        try:
            subprocess.run(f"make clean".split(),
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
        for src_dir in ["source", "src"]:
            if os.path.isfile(f"{example_dir}/{src_dir}/makefile"):
                clean_col = "clean"
                example_type = f"{src_dir}/Makefile"
                working_dir=f"{example_dir}/{src_dir}"

                try:
                    subprocess.run(f"make clean".split(),
                                   stdout=subprocess.DEVNULL,
                                   stderr=subprocess.DEVNULL,
                                   cwd=working_dir)

                except subprocess.CalledProcessError as err:
                    clean_col = "Already clean"

        for doc_config in ["documentation.config", "Doxyfile"]:
            if os.path.isfile(f"{example_dir}/{doc_config}"):
                doc_col = "Doxygen"
                subprocess.run(f"doxygen {doc_config}".split(),
                               stdout=subprocess.DEVNULL,
                               stderr=subprocess.DEVNULL,
                               cwd=example_dir)

    print(f"|{example_dir:<42}|{example_type:^18}|{clean_col:^10}|{doc_col:^10}|")


def prepare_zip_files(review_dirs: List[str], build_dir: str):
    """
    Generate one zip file for each specified top-level directory.

    # Args
        review_dirs: top-level review directory

        build_dir: directory in which to generate zip files
    """

    print()
    print("Zip File Generation")

    if not os.path.isdir(build_dir):
        os.mkdir(build_dir)

    for review_dir in review_dirs:
        zip_name = build_dir + "/" + review_dir.split("/")[-1] + ".zip"

        print(f"{review_dir:<42} -> {zip_name:<20}")

        with zipfile.ZipFile(zip_name, "w",
                             compression=zipfile.ZIP_DEFLATED,
                             compresslevel=9) as review_zip:

            for root_dir, _, files in os.walk(review_dir):
                for a_file in files:
                    review_zip.write(os.path.join(root_dir, a_file))


def main():
    parser = argparse.ArgumentParser(description="Perform Review Example deploy operations.")

    parser.add_argument("--workers",
                        dest="num_workers",
                        type=int,
                        default=4,
                        help="# of build/deploy workers")

    parser.add_argument("review_dir",
                        nargs=1,
                        type=str,
                        help="Base Review directory")

    parser.add_argument("--build-type",
                        dest="build_type",
                        nargs=1,
                        type=str,
                        choices=["review", "example"],
                        default="example",
                        help="How to queue tasks")

    parser.add_argument("--no-deploy",
                        dest="no_deploy",
                        action="store_true",
                        default=False,
                        help="Skip pre-deploy cleanup and doc generation")

    parser.add_argument("--no-zip",
                        dest="no_zip",
                        action="store_true",
                        default=False,
                        help="Skip Generation of zip files")

    parser.add_argument("--all-dirs",
                        dest="all_dirs",
                        action="store_true",
                        default=False,
                        help="Examine all top-level directories")

    args = parser.parse_args()
    base_review_dir = args.review_dir[0]
    build_dir = f"{base_review_dir}/build"

    if not args.all_dirs:
        review_dirs = glob.glob(f"{base_review_dir}/Review-*")
        example_dirs = glob.glob(f"{base_review_dir}/Review-*/Example*")

    else:
        review_dirs = [d for d in glob.glob(f"{base_review_dir}/Review-*")
                       if os.path.isdir(d)]

        example_dirs = glob.glob(f"{base_review_dir}/*/Example*")

    if not args.no_deploy:
        with ProcessPoolExecutor(max_workers=args.num_workers) as executor:
            if args.build_type == "review":
                for review_dir in review_dirs:
                    executor.submit(prepare_example_set, review_dir)
            else:
                for example_dir in example_dirs:
                    executor.submit(prepare_example, example_dir)

    if not args.no_zip:
        prepare_zip_files(review_dirs, build_dir)


if __name__ == "__main__":
    main()
