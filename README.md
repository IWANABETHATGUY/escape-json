# A faster implementation of escape json 
Faster JSON escape for `0x2028` and `0x2029`
## Related issues: 
1. https://bugs.chromium.org/p/v8/issues/detail?id=1907

## Benchmark result
array.json @`two_pass_replace`                                                                             
                        time:   [40.201 ns 40.336 ns 40.488 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

array.json @`escape_json`                                                                             
                        time:   [34.798 ns 34.853 ns 34.915 ns]
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe

array.json @`two_pass_search_one_pass_copy`                                                                             
                        time:   [22.131 ns 22.144 ns 22.157 ns]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

array.json @`regex`     time:   [68.002 ns 68.103 ns 68.210 ns]                                
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

array.json @`regex_iter`                                                                             
                        time:   [30.807 ns 30.898 ns 30.982 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

big.json @`two_pass_replace`                                                                            
                        time:   [391.22 µs 393.73 µs 397.41 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

Benchmarking big.json @`escape_json`: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.6s, enable flat sampling, or reduce sample count to 60.
big.json @`escape_json` time:   [1.2745 ms 1.2841 ms 1.2931 ms]                                     
Found 19 outliers among 100 measurements (19.00%)
  6 (6.00%) low severe
  9 (9.00%) high mild
  4 (4.00%) high severe

big.json @`two_pass_search_one_pass_copy`                                                                            
                        time:   [94.267 µs 94.435 µs 94.610 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

big.json @`regex`       time:   [50.135 µs 50.445 µs 50.783 µs]                              
Found 10 outliers among 100 measurements (10.00%)
  9 (9.00%) low mild
  1 (1.00%) high severe

big.json @`regex_iter`  time:   [51.376 µs 51.442 µs 51.514 µs]                                    
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

escape.json @`two_pass_replace`                                                                            
                        time:   [144.92 ns 145.26 ns 145.63 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

escape.json @`escape_json`                                                                            
                        time:   [111.47 ns 111.85 ns 112.27 ns]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

escape.json @`two_pass_search_one_pass_copy`                                                                            
                        time:   [81.143 ns 81.481 ns 81.863 ns]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

escape.json @`regex`    time:   [182.03 ns 182.50 ns 183.03 ns]                                 
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe

escape.json @`regex_iter`                                                                            
                        time:   [126.69 ns 126.86 ns 127.02 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

large.json @`two_pass_replace`                                                                            
                        time:   [60.101 µs 60.598 µs 61.149 µs]
Found 19 outliers among 100 measurements (19.00%)
  16 (16.00%) low severe
  2 (2.00%) high mild
  1 (1.00%) high severe

large.json @`escape_json`                                                                            
                        time:   [560.82 µs 560.96 µs 561.13 µs]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

large.json @`two_pass_search_one_pass_copy`                                                                             
                        time:   [44.765 µs 44.787 µs 44.811 µs]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

large.json @`regex`     time:   [27.461 µs 28.031 µs 28.517 µs]                                 

large.json @`regex_iter`                                                                             
                        time:   [20.581 µs 20.596 µs 20.619 µs]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

package.json @`two_pass_replace`                                                                            
                        time:   [136.15 ns 136.35 ns 136.56 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild

package.json @`escape_json`                                                                             
                        time:   [1.0753 µs 1.0767 µs 1.0781 µs]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe

package.json @`two_pass_search_one_pass_copy`                                                                            
                        time:   [114.70 ns 115.01 ns 115.29 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high severe

package.json @`regex`   time:   [99.359 ns 99.788 ns 100.30 ns]                                  
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

package.json @`regex_iter`                                                                            
                        time:   [55.876 ns 55.926 ns 55.980 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

escape_heavy.json @`two_pass_replace`                                                                             
                        time:   [1.7433 µs 1.7444 µs 1.7455 µs]
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe

escape_heavy.json @`escape_json`                                                                             
                        time:   [3.6184 µs 3.6239 µs 3.6307 µs]

escape_heavy.json @`two_pass_search_one_pass_copy`                                                                             
                        time:   [2.7976 µs 2.7987 µs 2.7998 µs]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

escape_heavy.json @`regex`                                                                             
                        time:   [4.1113 µs 4.1152 µs 4.1196 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

escape_heavy.json @`regex_iter`                                                                             
                        time:   [5.4050 µs 5.4183 µs 5.4314 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
```
Runs on AMD 5950x