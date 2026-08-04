# Kaadivisors - very fast algorithm for finding prime divisors of huge composite numbers.

`Algorithmic complexity: O(√n)` BUT **FASTER** than the famous crate [divisors](https://crates.io/crates/divisors)!

Even if we cannot reduce the time to find divisors when the number is prime, we can do it when our number is composite.
So, this program finds all **prime divisors** by iterating throught the numbers up to the <ins>root of the greatiest divisor</ins>.

Oh, and this program use generics, so everyone can pass <ins>any unsigned integer as an argument</ins> (u8, u16, u32, u64, u128)!

# Cargo.toml
```toml
[dependencies]
kaadivisors = "2.0"
```

# Example
```rust
use std::time::{Instant};
use kaadivisors::get_divisors;

fn main() {
    let number_u32: u32 = 1234567890;
    let start_u32 = Instant::now();
    let res_u32 = get_divisors(number_u32); // Vec<(u32, u8)>
    println!("Divisors (u32): {:?}\nFinished in {:?}", res_u32, start_u32.elapsed());

    let number_u128: u128 = 123456789012345678901234567890;
    let start_u128 = Instant::now();
    let res_u128 = get_divisors(number_u128); // Vec<(u128, u8)>
    println!("Divisors (u128): {:?}\nFinished in {:?}", res_u128, start_u128.elapsed());
}
```
**OUTPUT**: Vec<(divisor, power)>
```
Divisors (u32): [(2, 1), (3, 2), (5, 1), (3607, 1), (3803, 1)]
Finished in 2.219µs
Divisors (u128): [(2, 1), (3, 3), (5, 1), (7, 1), (13, 1), (31, 1), (37, 1), (211, 1), (241, 1), (2161, 1), (3607, 1), (3803, 1), (2906161, 1)]
Finished in 3.594µs
```

# Algorithm

* **Firstly**, we check divisibility by 2 and 3, and then iterate in increments of 6, since all prime divisors are of the form 6k ± 1.  
* **Secondly**, we iterate through the numbers until the first divisor of our number is found.  
In this case, we divide the number by it as much as possible (function: get_power), and then continue iterating through the numbers up to the <u>root of the result</u>.  
* **Back to the previous step**

# License
MIT  
Apache-2.0
