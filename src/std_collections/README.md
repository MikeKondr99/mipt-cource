# Comm utility

## Task

Implement the command line utility called `comm`. It takes two files as an input parameter and outputs all common lines, i.e., lines that appear in both files. Please note _every unique line should be printed once_. Output order doesn't matter.

## Implementation tips

- To get a command line arguments, use [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html):

    ```rust
    let args = std::env::args().collect::<Vec<String>>();
    ```

- To read file line by line, use [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html) and [`std::io::BufReader`](https://doc.rust-lang.org/stable/std/io/struct.BufReader.html):

    ```rust
    use std::{fs::File, io::BufRead, io::BufReader};

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        // ...
    }
    ```

- To intersect the lines, you can use [HashSet](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html). You'll need `insert`, `contains` and `take` functions.

# LRU Cache

In this problem, you'll write a cache with [Least Recently Used](https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU) strategy.

## Task

Implement the LRUCache structure:

- `LRUCache::new(capacity: usize) -> Self` - Initialize the LRU cache with some _positive_ size capacity.
- `LRUCache::get(&mut self, key: &K) -> Some(&V)` - Return the `Some(&value)` of the key if the key exists, otherwise return `None`.
- `LRUCache::insert(&mut self, key: K, value: V) -> Some(V)` - Update the value of the key and return `Some(value)` with old value if the key exists. Otherwise, add the key-value pair to the cache and return `None`. If the number of keys exceeds the capacity of this operation, evict the least recently used key.

You're not required to write the best O(1) implementation and for that `K` is clonable, hashable and comparable. The last two mean you can use `BTreeMap` and `HashMap`. However, they have to be something like O(log(N)) at least to pass the stress test. Check this test out!

## Questions

- Is there a way to implement it without `Clone` on `K`?
- Is it possible to get pure O(1) without unsafe?

# Longest common prefix

In this problem, you'll code a function to compute longest common prefix of multiple strings. You're allowed to make additional allocations.

## UTF-8

Remember: Rust strings are UTF-8! That means we'll consider them as input too. Moreover, it's not clear when two UTF-8 strings are equal.

Consider the word **café**:

This can be represented as:

`A = [U+0063 U+0061 U+0066 U+0065 U+0301]` (ends with **e** and a **combining accent**)

But also as

`B = [U+0063 U+0061 U+0066 U+00E9]` (ends with **é**, the combined form)

To be clear, we are searching for the longest prefix of equal [`.chars()`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.chars).

## Useful links

- [`.as_bytes()`](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes) method.
- [`.char_indices()`](https://doc.rust-lang.org/std/string/struct.String.html#method.char_indices) method.

## Complexity

Your solution must be O(n), where N is the total length of the strings. Beware of [`.nth()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth): it's linear!

## Advanced level

- Write a solution that doesn't allocate anything.
