# A faster implementation of escape json 
Faster JSON escape for `0x2028` and `0x2029`
## Related issues: 
1. https://bugs.chromium.org/p/v8/issues/detail?id=1907

## Benchmark result
```
group                                                 base                                   
-----                                                 ----                                   
array.json @`escape_json`                             1.00     36.5±0.89ns        ? ?/sec    
array.json @`memchr3`                                 1.00     49.4±0.53ns        ? ?/sec    
array.json @`regex_iter`                              1.00     33.9±1.11ns        ? ?/sec    
array.json @`regex`                                   1.00     68.8±0.57ns        ? ?/sec    
array.json @`two_pass_replace`                        1.00     38.2±1.27ns        ? ?/sec    
array.json @`two_pass_search_one_pass_copy`           1.00     22.0±0.46ns        ? ?/sec    

big.json @`escape_json`                               1.00  1284.0±37.21µs        ? ?/sec    
big.json @`memchr3`                                   1.00     33.3±0.50µs        ? ?/sec    
big.json @`regex_iter`                                1.00     49.8±0.64µs        ? ?/sec    
big.json @`regex`                                     1.00     52.5±0.90µs        ? ?/sec    
big.json @`two_pass_replace`                          1.00   402.9±11.61µs        ? ?/sec    
big.json @`two_pass_search_one_pass_copy`             1.00    102.7±1.32µs        ? ?/sec    

escape.json @`escape_json`                            1.00    112.1±1.27ns        ? ?/sec    
escape.json @`memchr3`                                1.00     95.9±2.25ns        ? ?/sec    
escape.json @`regex_iter`                             1.00    127.2±2.45ns        ? ?/sec    
escape.json @`regex`                                  1.00    169.8±3.32ns        ? ?/sec    
escape.json @`two_pass_replace`                       1.00    133.0±6.02ns        ? ?/sec    
escape.json @`two_pass_search_one_pass_copy`          1.00     79.0±2.65ns        ? ?/sec    

escape_heavy.json @`escape_json`                      1.00      3.8±0.01µs        ? ?/sec    
escape_heavy.json @`memchr3`                          1.00      2.8±0.06µs        ? ?/sec    
escape_heavy.json @`regex_iter`                       1.00      5.2±0.15µs        ? ?/sec    
escape_heavy.json @`regex`                            1.00      4.7±0.02µs        ? ?/sec   
escape_heavy.json @`two_pass_replace`                 1.00  1699.2±10.00ns        ? ?/sec  
escape_heavy.json @`two_pass_search_one_pass_copy`    1.00      2.8±0.12µs        ? ?/sec 

large.json @`escape_json`                             1.00   548.0±21.22µs        ? ?/sec
large.json @`memchr3`                                 1.00     14.6±0.35µs        ? ?/sec    
large.json @`regex_iter`                              1.00     22.0±0.27µs        ? ?/sec    
large.json @`regex`                                   1.00     24.2±0.57µs        ? ?/sec    
large.json @`two_pass_replace`                        1.00     64.2±0.85µs        ? ?/sec    
large.json @`two_pass_search_one_pass_copy`           1.00     45.5±0.45µs        ? ?/sec    

package.json @`escape_json`                           1.00  1087.6±10.59ns        ? ?/sec    
package.json @`memchr3`                               1.00     72.2±0.67ns        ? ?/sec    
package.json @`regex_iter`                            1.00     59.2±1.47ns        ? ?/sec    
package.json @`regex`                                 1.00     98.2±1.56ns        ? ?/sec    
package.json @`two_pass_replace`                      1.00    141.0±1.00ns        ? ?/sec   
package.json @`two_pass_search_one_pass_copy`         1.00    117.8±0.82ns        ? ?/sec   
```
Runs on AMD 5950x