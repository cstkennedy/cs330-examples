| Command                                                                           | Mean [ms]         | Min [ms] | Max [ms] |
| :---                                                                              | ---:              | ---:     | ---:     |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 1 10`                 | 51.9 ± 4.6        | 50.8     | 79.3     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 1 10`         | 127.3 ± 8.7       | 120.5    | 141.4    |
| `Review-11-Rust-Threads/target/release/coin-flip 1 10`                            | 1.3 ± 0.7         | 0.2      | 5.2      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 2 10`                 | 59.7 ± 0.8        | 58.4     | 61.3     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 2 10`         | 137.8 ± 8.5       | 131.4    | 152.4    |
| `Review-11-Rust-Threads/target/release/coin-flip 2 10`                            | 1.4 ± 1.0         | 0.3      | 5.0      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 3 10`                 | 60.7 ± 3.1        | 59.5     | 78.6     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 3 10`         | 138.6 ± 7.9       | 132.7    | 154.2    |
| `Review-11-Rust-Threads/target/release/coin-flip 3 10`                            | 1.1 ± 0.4         | 0.3      | 3.4      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 4 10`                 | 61.6 ± 1.9        | 60.6     | 72.8     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 4 10`         | 138.2 ± 6.7       | 133.5    | 154.0    |
| `Review-11-Rust-Threads/target/release/coin-flip 4 10`                            | 0.7 ± 0.2         | 0.3      | 1.4      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 1 100`                | 51.3 ± 0.3        | 51.0     | 53.6     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 1 100`        | 126.1 ± 10.4      | 120.2    | 155.2    |
| `Review-11-Rust-Threads/target/release/coin-flip 1 100`                           | 1.3 ± 0.7         | 0.3      | 5.1      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 2 100`                | 59.5 ± 1.6        | 58.5     | 68.9     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 2 100`        | 142.1 ± 11.3      | 131.6    | 165.5    |
| `Review-11-Rust-Threads/target/release/coin-flip 2 100`                           | 1.6 ± 0.8         | 0.4      | 3.7      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 3 100`                | 60.6 ± 1.5        | 59.2     | 69.1     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 3 100`        | 137.7 ± 7.2       | 132.5    | 152.6    |
| `Review-11-Rust-Threads/target/release/coin-flip 3 100`                           | 0.8 ± 0.2         | 0.4      | 1.8      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 4 100`                | 61.5 ± 0.9        | 60.6     | 65.3     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 4 100`        | 145.5 ± 11.2      | 133.8    | 169.5    |
| `Review-11-Rust-Threads/target/release/coin-flip 4 100`                           | 0.6 ± 0.2         | 0.3      | 2.1      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 1 1000`               | 52.1 ± 0.2        | 51.7     | 52.5     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 1 1000`       | 126.8 ± 9.1       | 120.0    | 141.2    |
| `Review-11-Rust-Threads/target/release/coin-flip 1 1000`                          | 1.4 ± 0.7         | 0.3      | 5.4      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 2 1000`               | 60.3 ± 2.4        | 59.1     | 74.6     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 2 1000`       | 138.0 ± 7.6       | 131.5    | 152.6    |
| `Review-11-Rust-Threads/target/release/coin-flip 2 1000`                          | 1.7 ± 0.9         | 0.4      | 5.3      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 3 1000`               | 61.0 ± 2.7        | 59.8     | 77.1     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 3 1000`       | 142.9 ± 9.0       | 132.2    | 154.2    |
| `Review-11-Rust-Threads/target/release/coin-flip 3 1000`                          | 0.7 ± 0.2         | 0.3      | 2.9      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 4 1000`               | 61.9 ± 2.1        | 60.6     | 74.0     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 4 1000`       | 141.9 ± 8.9       | 133.3    | 154.6    |
| `Review-11-Rust-Threads/target/release/coin-flip 4 1000`                          | 0.7 ± 0.2         | 0.3      | 2.1      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 1 10000`              | 60.9 ± 0.6        | 60.5     | 64.6     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 1 10000`      | 134.2 ± 11.0      | 120.3    | 149.6    |
| `Review-11-Rust-Threads/target/release/coin-flip 1 10000`                         | 1.5 ± 0.9         | 0.3      | 4.7      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 2 10000`              | 64.4 ± 4.0        | 63.3     | 86.7     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 2 10000`      | 138.1 ± 8.4       | 131.6    | 151.6    |
| `Review-11-Rust-Threads/target/release/coin-flip 2 10000`                         | 1.2 ± 0.6         | 0.4      | 2.8      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 3 10000`              | 63.6 ± 1.4        | 62.8     | 71.8     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 3 10000`      | 138.6 ± 8.7       | 133.0    | 162.9    |
| `Review-11-Rust-Threads/target/release/coin-flip 3 10000`                         | 1.1 ± 0.3         | 0.4      | 2.3      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 4 10000`              | 63.5 ± 1.2        | 62.6     | 69.1     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 4 10000`      | 140.0 ± 8.5       | 133.8    | 157.2    |
| `Review-11-Rust-Threads/target/release/coin-flip 4 10000`                         | 0.6 ± 0.2         | 0.3      | 2.5      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 1 100000`             | 147.3 ± 0.6       | 146.4    | 148.9    |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 1 100000`     | 127.1 ± 8.5       | 120.6    | 140.9    |
| `Review-11-Rust-Threads/target/release/coin-flip 1 100000`                        | 1.0 ± 0.4         | 0.3      | 2.2      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 2 100000`             | 107.3 ± 1.1       | 106.2    | 112.0    |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 2 100000`     | 140.6 ± 11.2      | 131.8    | 172.7    |
| `Review-11-Rust-Threads/target/release/coin-flip 2 100000`                        | 1.3 ± 0.5         | 0.4      | 2.1      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 3 100000`             | 92.0 ± 0.4        | 90.9     | 92.8     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 3 100000`     | 140.0 ± 11.0      | 132.4    | 174.6    |
| `Review-11-Rust-Threads/target/release/coin-flip 3 100000`                        | 0.8 ± 0.2         | 0.4      | 1.5      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 4 100000`             | 84.6 ± 0.5        | 84.0     | 86.4     |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 4 100000`     | 141.8 ± 9.2       | 133.8    | 155.5    |
| `Review-11-Rust-Threads/target/release/coin-flip 4 100000`                        | 0.9 ± 0.3         | 0.4      | 2.3      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 1 1000000`            | 1014.9 ± 5.1      | 1007.7   | 1023.3   |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 1 1000000`    | 132.6 ± 10.7      | 124.2    | 157.2    |
| `Review-11-Rust-Threads/target/release/coin-flip 1 1000000`                       | 1.5 ± 0.2         | 1.3      | 2.3      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 2 1000000`            | 509.2 ± 61.7      | 368.1    | 540.8    |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 2 1000000`    | 140.6 ± 8.8       | 133.7    | 160.3    |
| `Review-11-Rust-Threads/target/release/coin-flip 2 1000000`                       | 1.2 ± 0.1         | 0.8      | 1.8      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 3 1000000`            | 378.3 ± 1.5       | 376.5    | 381.1    |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 3 1000000`    | 143.2 ± 10.6      | 133.9    | 168.4    |
| `Review-11-Rust-Threads/target/release/coin-flip 3 1000000`                       | 1.0 ± 0.1         | 0.7      | 2.7      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 4 1000000`            | 291.9 ± 27.8      | 212.9    | 304.1    |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 4 1000000`    | 143.3 ± 9.1       | 135.2    | 163.4    |
| `Review-11-Rust-Threads/target/release/coin-flip 4 1000000`                       | 1.0 ± 0.1         | 0.6      | 1.7      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 1 10000000`           | 9648.1 ± 68.1     | 9576.1   | 9768.7   |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 1 10000000`   | 168.8 ± 8.9       | 162.2    | 182.6    |
| `Review-11-Rust-Threads/target/release/coin-flip 1 10000000`                      | 10.0 ± 0.2        | 9.8      | 10.9     |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 2 10000000`           | 4734.8 ± 278.5    | 3943.6   | 4852.0   |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 2 10000000`   | 163.9 ± 9.0       | 153.2    | 173.6    |
| `Review-11-Rust-Threads/target/release/coin-flip 2 10000000`                      | 5.4 ± 0.1         | 5.2      | 6.5      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 3 10000000`           | 3247.8 ± 19.3     | 3220.2   | 3280.9   |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 3 10000000`   | 156.5 ± 8.6       | 147.3    | 167.8    |
| `Review-11-Rust-Threads/target/release/coin-flip 3 10000000`                      | 3.9 ± 0.4         | 3.6      | 6.4      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 4 10000000`           | 2442.6 ± 11.5     | 2423.7   | 2467.6   |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 4 10000000`   | 150.6 ± 8.4       | 144.1    | 165.3    |
| `Review-11-Rust-Threads/target/release/coin-flip 4 10000000`                      | 4.0 ± 0.8         | 2.9      | 4.9      |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 1 100000000`          | 96099.3 ± 393.9   | 95488.5  | 96729.3  |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 1 100000000`  | 549.8 ± 13.4      | 538.7    | 581.8    |
| `Review-11-Rust-Threads/target/release/coin-flip 1 100000000`                     | 95.3 ± 0.1        | 95.1     | 95.5     |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 2 100000000`          | 47201.6 ± 1041.0  | 45037.6  | 48263.3  |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 2 100000000`  | 352.7 ± 10.3      | 342.9    | 365.5    |
| `Review-11-Rust-Threads/target/release/coin-flip 2 100000000`                     | 50.7 ± 7.7        | 48.0     | 82.4     |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 3 100000000`          | 31988.4 ± 124.5   | 31768.0  | 32243.5  |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 3 100000000`  | 283.9 ± 10.4      | 275.3    | 301.0    |
| `Review-11-Rust-Threads/target/release/coin-flip 3 100000000`                     | 33.7 ± 4.8        | 32.2     | 55.6     |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 4 100000000`          | 23921.9 ± 171.6   | 23744.3  | 24335.5  |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 4 100000000`  | 251.0 ± 10.3      | 240.2    | 263.1    |
| `Review-11-Rust-Threads/target/release/coin-flip 4 100000000`                     | 32.1 ± 6.8        | 24.3     | 42.1     |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 1 1000000000`         | 959770.9 ± 6481.1 | 951526.4 | 969532.8 |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 1 1000000000` | 4351.5 ± 48.5     | 4291.4   | 4468.3   |
| `Review-11-Rust-Threads/target/release/coin-flip 1 1000000000`                    | 948.7 ± 1.6       | 947.7    | 953.2    |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 2 1000000000`         | 478081.3 ± 6077.1 | 472612.8 | 493334.8 |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 2 1000000000` | 2262.8 ± 10.1     | 2249.5   | 2275.6   |
| `Review-11-Rust-Threads/target/release/coin-flip 2 1000000000`                    | 478.6 ± 5.4       | 474.5    | 492.8    |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 3 1000000000`         | 318087.2 ± 1040.0 | 316818.5 | 320215.7 |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 3 1000000000` | 1586.6 ± 43.1     | 1555.1   | 1704.3   |
| `Review-11-Rust-Threads/target/release/coin-flip 3 1000000000`                    | 333.8 ± 30.5      | 317.3    | 416.9    |
| `python3.11 Review-11-Python-Threads/Example-1/coin_flip.py 4 1000000000`         | 238887.0 ± 2009.8 | 237041.7 | 242512.5 |
| `python3.11 ./Review-11-Python-Threads/Example-1-NumPy/coin_flip.py 4 1000000000` | 1201.5 ± 31.4     | 1125.9   | 1242.3   |
| `Review-11-Rust-Threads/target/release/coin-flip 4 1000000000`                    | 273.7 ± 49.6      | 238.1    | 351.1    |
