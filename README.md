# Divisors
Very FAST code for finding divisors of a number (Rust, C++, Python)  
[![License](https://img.shields.io/badge/Licrnse-%20Apache%202.0-green.svg)](#)


### MANUAL
The first step: enter your number.  
The second step: the program will print all divisors.  

### CONSTRAINTS
| Language | Constraints |
| --- | --- |
| Rust | 0 <= your number < 2<sup>128</sup> |
| C++ | 0 <= your number < 2<sup>64</sup> |
| Python | Any unsigned number (int) |

### OUTPUT FORMAT
```bash
   This program returns divisors of a number.
   Enter the number:
   1234567890            # YOUR NUMBER
   ----------
   2 ^1                  # Divisor
   3 ^2                  # Divisor
   5 ^1                  # Divisor
   3607 ^1               # Divisor
   3803 ^1               # Divisor
   ----------
   Finished: 118.353µs   # Execution time
```
All divisors are PRIME NUMBERS (2, 3, 5, 3607, 3803)

`3 ^2` - here `3` is a <ins>prime divisor</ins>, `^2` is a <ins>power</ins> (3<sup>2</sup> = 3 × 3).

Indeed, 2 × 3<sup>2</sup> × 5 × 3607 × 3803 = 1234567890
