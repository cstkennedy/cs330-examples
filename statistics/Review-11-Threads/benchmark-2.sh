hyperfine \
    --export-markdown coin_flip-2.md  \
    --parameter-list num_threads 1,2,3,4 \
    --parameter-list num_flips 10000000000 \
    "python3.11 ../../Review-11-Python-Threads/Example-1-NumPy/coin_flip.py {num_threads} {num_flips}" \
    "../../Review-11-Rust-Threads/target/release/coin-flip {num_threads} {num_flips}"

    # [Too Slow] # "python3.11 ../../Review-11-Python-Threads/Example-1/coin_flip.py {num_threads} {num_flips}" \


