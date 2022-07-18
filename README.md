## Benchmark result
```bash
Gnuplot not found, using plotters backend
array.json @`two_pass_replace`                                                                             
                        time:   [42.110 ns 42.247 ns 42.388 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

array.json @`escape_json`                                                                             
                        time:   [30.631 ns 30.889 ns 31.177 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

array.json @`two_pass_search_one_pass_copy`                                                                             
                        time:   [21.956 ns 21.977 ns 21.999 ns]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

big.json @`two_pass_replace`                                                                            
                        time:   [412.50 µs 412.86 µs 413.22 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking big.json @`escape_json`: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.4s, enable flat sampling, or reduce sample count to 60.
big.json @`escape_json` time:   [1.2696 ms 1.2787 ms 1.2885 ms]                                     

big.json @`two_pass_search_one_pass_copy`                                                                            
                        time:   [117.84 µs 118.34 µs 118.92 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

escape.json @`two_pass_replace`                                                                            
                        time:   [137.12 ns 137.29 ns 137.49 ns]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

escape.json @`escape_json`                                                                            
                        time:   [111.57 ns 111.88 ns 112.19 ns]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high severe

escape.json @`two_pass_search_one_pass_copy`                                                                            
                        time:   [78.805 ns 78.943 ns 79.096 ns]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

large.json @`two_pass_replace`                                                                            
                        time:   [72.211 µs 72.253 µs 72.296 µs]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe

large.json @`escape_json`                                                                            
                        time:   [575.99 µs 576.30 µs 576.63 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

large.json @`two_pass_search_one_pass_copy`                                                                            
                        time:   [55.314 µs 55.422 µs 55.548 µs]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) low mild
  1 (1.00%) high mild
  4 (4.00%) high severe

package.json @`two_pass_replace`                                                                            
                        time:   [151.78 ns 152.17 ns 152.75 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

package.json @`escape_json`                                                                             
                        time:   [1.0227 µs 1.0273 µs 1.0312 µs]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  5 (5.00%) low mild

package.json @`two_pass_search_one_pass_copy`                                                                            
                        time:   [123.56 ns 123.64 ns 123.72 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

escape_heavy.json @`two_pass_replace`                                                                             
                        time:   [1.5667 µs 1.5697 µs 1.5738 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

escape_heavy.json @`escape_json`                                                                             
                        time:   [4.0174 µs 4.0192 µs 4.0209 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

escape_heavy.json @`two_pass_search_one_pass_copy`                                                                             
                        time:   [2.7206 µs 2.7220 µs 2.7234 µs]
```
Runs on AMD 5950x