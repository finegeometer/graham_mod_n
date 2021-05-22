/// Calculate `(base ^ exponent) % modulus`.
/// Uses the method of repeated squaring.
/// `O(log n)` time.
fn modexp(base: u32, mut exponent: u32, modulus: u32) -> u32 {
    let mut base = base as u64;
    let modulus = modulus as u64;
    let mut out = 1;
    while exponent > 0 {
        if exponent & 1 > 0 {
            out *= base;
            out %= modulus;
        }
        base *= base;
        base %= modulus;
        exponent >>= 1;
    }
    out as u32
}

/// Calculate the totient function of the numbers in `0..n`.
/// Algorithm from https://www.geeksforgeeks.org/eulers-totient-function-for-all-numbers-smaller-than-or-equal-to-n/
///
/// Executes in `O(n * log log n)` time, and in `4n + O(1)` bytes of memory.
fn totient_table(n: u32) -> Vec<u32> {
    // This is the only allocation in the entire program.
    let mut out: Vec<u32> = (0..n).collect();

    // Outer loop executes in time `sum of O(n/p), for primes p < n`.
    // This is `O(n * log log n)`.
    for p in 2..n {
        if out[p as usize] == p {
            // `p` is prime.
            // If it weren't, `out[p]` would have been decreased by the inner loop
            // when `p` was a prime factor of its current value.
            out[p as usize] -= 1;

            // Inner loop executes in time `O(n/p)`.
            let mut i = 2 * p;
            while i < n {
                out[i as usize] -= out[i as usize] / p;
                i += p;
            }
        }
    }
    out
}

/// Calculate a table of Graham's number mod `n`.
/// Executes in `O(max * log max)` time, and in `4*max + O(1)` bytes of memory.
fn graham_table(max: u32) -> Vec<u32> {
    // This buffer is fairly complicated, due to memory optimizations.
    // The algorithm below iterates through each index, in order.
    // When the algorithm is at index `n`, the contents of the buffer will be the following:
    //
    // - `buf[i]`, for `i < n`, will contain `G % i`,
    //   where `G` is any sufficiently tall power-tower of threes.
    //
    // - `buf[i]`, for `n <= i < 3n && i % 3 == 0`, will contain a number `a`
    //   such that for sufficiently large `k`, `a * 3^k ≡ 3^(k-1) mod (i/3)`.
    //
    // - `buf[i]`, for any other `i`, will contain `totient(i)`.
    let mut buf = totient_table(max);

    // The algorithm starts at `n=2`, so we need to do a little setup.
    buf[1] = 0; // `G % 1 == 0`
    buf[3] = 0; // `∀k>0. 0 * 3^k ≡ 3^(k-1) mod 1`

    for n in 2..max {
        if n % 3 == 0 {
            // For sufficiently large `k`, `a * 3^k ≡ 3^(k-1) mod (n/3)`.
            let a = buf[n as usize] as u64;

            // `G % (n/3)`
            let g_mod_n_thirds = buf[(n / 3) as usize] as u64;

            // ```text
            // G % n
            // = (3 * (G/3)) % (3 * (n/3))
            // = 3 * ((G/3) % (n/3))
            // = 3 * (a * (G % (n/3)) % (n/3))
            // ```
            let g_mod_n = 3 * ((a * g_mod_n_thirds) % (n / 3) as u64);

            buf[n as usize] = g_mod_n as u32;
            if n < max / 3 {
                // For sufficiently large `k`,
                // ```
                // (a * 3^k) % n
                // = (3 * a * 3^(k-1)) % (3 * (n/3))
                // = 3 * ((a * 3^(k-1)) % (n/3))
                // = 3 * (3^(k-2) % (n/3))
                // = 3^(k-1) % n
                // ```
                buf[3 * n as usize] = a as u32;
            }
        } else {
            // Let `G = 3^G₂`. `G₂` is also a long power-tower of threes.

            // [Euler's Theorem](https://en.wikipedia.org/wiki/Euler%27s_theorem)
            // implies that `3^(totient(n)) ≡ 1 mod n`.
            // Therefore, `G % n = 3^G₂ % n = 3^(G₂ % totient(n)) % n`.
            let totient_n = buf[n as usize];
            buf[n as usize] = modexp(3, buf[totient_n as usize], n);

            if n < max / 3 {
                // Furthermore, `3^(totient(n)-1)` is the inverse of three, mod `n`.
                // So it satisfies the property "For sufficiently large `k`, `a * 3^k ≡ 3^(k-1) mod (i/3)`".
                buf[3 * n as usize] = modexp(3, totient_n - 1, n);
            }
        }
    }

    // Now, `buf[i] = G % i` throughout the entire buffer.
    // Remember that `G` can be any sufficiently tall power-tower of threes.
    // Graham's number is an unimaginably tall power-tower of threes,
    // so `buf[i] = Graham's number % i`.
    buf
}

fn main() {
    std::fs::write("graham_mod_n", bytemuck::cast_slice(&graham_table(1 << 30))).unwrap();
}
