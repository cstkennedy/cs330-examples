hyperfine \
    -i \
    --export-markdown coin_flip-1-weak.md  \
    --parameter-list num_threads 1,2,3,4 \
    --parameter-list num_flips 0,00,000,0000,00000,000000,0000000,00000000,000000000 \
    "python3.11 ../../Review-11-Python-Threads/Example-1/coin_flip.py {num_threads} {num_threads}{num_flips}" \
    "python3.11 ../../Review-11-Python-Threads/Example-1-NumPy/coin_flip.py {num_threads} {num_threads}{num_flips}" \
    "../../Review-11-Rust-Threads/target/release/coin-flip {num_threads} {num_threads}{num_flips}"
