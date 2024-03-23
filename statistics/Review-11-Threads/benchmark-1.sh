hyperfine \
    --export-markdown coin_flip-1.md  \
    --parameter-list num_threads 1,2,3,4 \
    --parameter-list num_flips 10,100,1000,10000,100000,1000000,10000000,100000000,1000000000 \
    "python3.11 ../../Review-11-Python-Threads/Example-1/coin_flip.py {num_threads} {num_flips}" \
    "python3.11 ../../Review-11-Python-Threads/Example-1-NumPy/coin_flip.py {num_threads} {num_flips}" \
    "../../Review-11-Rust-Threads/target/release/coin-flip {num_threads} {num_flips}"
