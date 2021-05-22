Calculates Graham's number modulo the first billion integers.

Takes a few minutes to compute, and uses 4GiB of RAM.
Spits the result to the file graham_mod_n,
as consecutive `u32`s, in your computer's native endianness.

I wrote this program because I wanted to know if `G + 4` is prime.
Spoiler: it isn't. Its smallest prime factor is `61_094_071`.
