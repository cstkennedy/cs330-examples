
# Raw Results

| Binary           | Input File                 | Mean [ms]      | Min [ms] | Max [ms] | Relative         |
| :---             | ---:                       | ---:           | ---:     | ---:     | ---:             |
| Example-5        | inputShapes.txt            | 5.7 ± 0.6      | 5.2      | 11.5     | 1.00 ± 0.14      |
| Example-5        | random-shapes-100.txt      | 7.0 ± 0.6      | 6.4      | 10.9     | 1.22 ± 0.16      |
| Example-5        | random-shapes-1000.txt     | 17.8 ± 1.0     | 16.9     | 24.4     | 3.11 ± 0.34      |
| Example-5        | random-shapes-10000.txt    | 125.6 ± 11.0   | 118.2    | 169.2    | 21.96 ± 2.80     |
| Example-5        | random-shapes-100000.txt   | 1143.0 ± 11.7  | 1133.4   | 1172.9   | 199.89 ± 18.60   |
| Example-5        | random-shapes-1000000.txt  | 11415.4 ± 70.2 | 11310.9  | 11544.6  | 1996.41 ± 185.08 |
| Buffered         | inputShapes.txt            | 5.7 ± 0.5      | 5.1      | 8.4      | 1.00             |
| Buffered         | random-shapes-100.txt      | 6.2 ± 0.6      | 5.5      | 9.3      | 1.08 ± 0.15      |
| Buffered         | random-shapes-1000.txt     | 7.9 ± 1.4      | 7.1      | 27.8     | 1.37 ± 0.27      |
| Buffered         | random-shapes-10000.txt    | 25.8 ± 1.8     | 24.7     | 39.7     | 4.52 ± 0.52      |
| Buffered         | random-shapes-100000.txt   | 196.8 ± 1.9    | 193.5    | 201.5    | 34.41 ± 3.20     |
| Buffered         | random-shapes-1000000.txt  | 1956.9 ± 20.8  | 1926.2   | 1986.9   | 342.24 ± 31.87   |
| Buffered + Rayon | inputShapes.txt`           | 6.8 ± 0.4      | 6.1      | 9.2      | 1.00             |
| Buffered + Rayon | random-shapes-100.txt`     | 7.2 ± 0.6      | 6.4      | 10.7     | 1.06 ± 0.11      |
| Buffered + Rayon | random-shapes-1000.txt`    | 9.2 ± 0.6      | 8.3      | 11.8     | 1.36 ± 0.12      |
| Buffered + Rayon | random-shapes-10000.txt`   | 26.5 ± 0.7     | 25.5     | 30.3     | 3.93 ± 0.27      |
| Buffered + Rayon | random-shapes-100000.txt`  | 193.9 ± 1.9    | 191.2    | 197.8    | 28.72 ± 1.85     |
| Buffered + Rayon | random-shapes-1000000.txt` | 1878.5 ± 26.6  | 1842.6   | 1916.2   | 278.21 ± 18.12   |


# Averages

| Binary           | Input File                 | Mean [ms]      |
| :---             | ---:                       | ---:           |
| Example-5        | inputShapes.txt            | 5.7 ± 0.6      |
| Example-5        | random-shapes-100.txt      | 7.0 ± 0.6      |
| Example-5        | random-shapes-1000.txt     | 17.8 ± 1.0     |
| Example-5        | random-shapes-10000.txt    | 125.6 ± 11.0   |
| Example-5        | random-shapes-100000.txt   | 1143.0 ± 11.7  |
| Example-5        | random-shapes-1000000.txt  | 11415.4 ± 70.2 |
| Buffered         | inputShapes.txt            | 5.7 ± 0.5      |
| Buffered         | random-shapes-100.txt      | 6.2 ± 0.6      |
| Buffered         | random-shapes-1000.txt     | 7.9 ± 1.4      |
| Buffered         | random-shapes-10000.txt    | 25.8 ± 1.8     |
| Buffered         | random-shapes-100000.txt   | 196.8 ± 1.9    |
| Buffered         | random-shapes-1000000.txt  | 1956.9 ± 20.8  |
| Buffered + Rayon | inputShapes.txt`           | 6.8 ± 0.4      |
| Buffered + Rayon | random-shapes-100.txt`     | 7.2 ± 0.6      |
| Buffered + Rayon | random-shapes-1000.txt`    | 9.2 ± 0.6      |
| Buffered + Rayon | random-shapes-10000.txt`   | 26.5 ± 0.7     |
| Buffered + Rayon | random-shapes-100000.txt`  | 193.9 ± 1.9    |
| Buffered + Rayon | random-shapes-1000000.txt` | 1878.5 ± 26.6  |


# Side-by-Side Averages

| Input File                | Example-5 [ms] | Buffered [ms] | Buffered + Rayon [ms] |
| :---                      | ---:           | ---:          | ---:                  |
| inputShapes.txt           | 5.7 ± 0.6      | 5.7 ± 0.5     | 6.8 ± 0.4             |
| random-shapes-100.txt     | 7.0 ± 0.6      | 6.2 ± 0.6     | 7.2 ± 0.6             |
| random-shapes-1000.txt    | 17.8 ± 1.0     | 7.9 ± 1.4     | 9.2 ± 0.6             |
| random-shapes-10000.txt   | 125.6 ± 11.0   | 25.8 ± 1.8    | 26.5 ± 0.7            |
| random-shapes-100000.txt  | 1143.0 ± 11.7  | 196.8 ± 1.9   | 193.9 ± 1.9           |
| random-shapes-1000000.txt | 11415.4 ± 70.2 | 1956.9 ± 20.8 | 1878.5 ± 26.6         |
