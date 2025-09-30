# LRU Cache Visualization Scaffold

This crate provides the Bevy scene for demonstrating an LRU cache driven by a hashmap + doubly linked list. It renders the cache storage, a recency deque, and an access log panel so you can animate hits, misses, and evictions.

## Running

```sh
cargo run
```

On launch you’ll see cache slots labeled with key/value pairs, a top bar showing recent accesses, and a queue at the bottom for eviction order. Wire in your algorithmic logic to highlight hits, promote nodes to the front, and evict the least recently used entry.
