| Command                                                 | Mean [ms]     | Min [ms] | Max [ms] | Relative       |
| :---                                                    | ---:          | ---:     | ---:     | ---:           |
| `./target/release/shapes_bin inputShapes.txt`           | 6.8 ± 0.4     | 6.1      | 9.2      | 1.00           |
| `./target/release/shapes_bin random-shapes-100.txt`     | 7.2 ± 0.6     | 6.4      | 10.7     | 1.06 ± 0.11    |
| `./target/release/shapes_bin random-shapes-1000.txt`    | 9.2 ± 0.6     | 8.3      | 11.8     | 1.36 ± 0.12    |
| `./target/release/shapes_bin random-shapes-10000.txt`   | 26.5 ± 0.7    | 25.5     | 30.3     | 3.93 ± 0.27    |
| `./target/release/shapes_bin random-shapes-100000.txt`  | 193.9 ± 1.9   | 191.2    | 197.8    | 28.72 ± 1.85   |
| `./target/release/shapes_bin random-shapes-1000000.txt` | 1878.5 ± 26.6 | 1842.6   | 1916.2   | 278.21 ± 18.12 |
