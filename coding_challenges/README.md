### **Progressively structured Rust coding exercise suite** designed to evaluate **logical thinking, problem solving, code quality, and system design** — moving from beginner to senior level.

The exercises intentionally test:

- Algorithmic reasoning
- Edge case handling
- Rust ownership & borrowing
- Error handling
- Concurrency
- Architecture & design tradeoffs

---

# 🟢 Level 1 — Beginner: Logical Foundations

## 1️⃣ FizzBuzz With a Twist

### Problem

Write a function:

```rust
fn fizzbuzz(n: u32) -> Vec<String>
```

Rules:

- Multiples of 3 → `"Fizz"`
- Multiples of 5 → `"Buzz"`
- Multiples of both → `"FizzBuzz"`
- Otherwise → the number
- If number contains digit `3`, append `"Lucky"`

Example:

```
15 → "FizzBuzz"
13 → "13Lucky"
30 → "FizzLucky"
```

### What This Tests

- Basic control flow
- String manipulation
- Conditional logic
- Clean function structure

---

## 2️⃣ Balanced Brackets Validator

### Problem

Implement:

```rust
fn is_valid(input: &str) -> bool
```

Validate brackets:

```
() {} []
```

Edge cases:

- Empty string
- Nested brackets
- Interleaved brackets

Example:

```
"{[()]}" → true
"{[(])}" → false
```

### What This Tests

- Stack usage
- Iteration over characters
- Logical correctness

---

# 🟡 Level 2 — Intermediate: Data Structures & Ownership

## 3️⃣ LRU Cache (Core Rust Skills)

### Problem

Implement a simple LRU Cache:

```rust
struct LruCache<K, V> { ... }

impl<K, V> LruCache<K, V> {
    fn new(capacity: usize) -> Self;
    fn get(&mut self, key: &K) -> Option<&V>;
    fn put(&mut self, key: K, value: V);
}
```

Requirements:

- O(1) get and put
- Evict least recently used
- No external crates

### What This Tests

- HashMap
- Doubly linked list
- Ownership management
- Borrow checker understanding

---

## 4️⃣ Word Frequency Counter (Streaming)

### Problem

Given a large text file, count word frequency efficiently.

Constraints:

- File may not fit in memory
- Case insensitive
- Ignore punctuation

Expected output:

```
word → count
```

### What This Tests

- Iterators
- Efficient IO
- HashMap
- Memory considerations

Bonus:

- Return top K frequent words

---

# 🟠 Level 3 — Advanced: Concurrency & System Thinking

## 5️⃣ Thread Pool Implementation

### Problem

Build a simple thread pool:

```rust
struct ThreadPool { ... }

impl ThreadPool {
    fn new(size: usize) -> Self;
    fn execute<F>(&self, job: F)
        where F: FnOnce() + Send + 'static;
}
```

Requirements:

- Worker threads wait for jobs
- Use channels
- Graceful shutdown

### What This Tests

- `std::thread`
- `Arc`, `Mutex`
- Channels
- Safe concurrency patterns

---

## 6️⃣ Rate Limiter (Token Bucket)

### Problem

Implement a token bucket rate limiter:

```rust
struct RateLimiter { ... }

impl RateLimiter {
    fn new(capacity: usize, refill_rate: usize) -> Self;
    fn allow(&mut self) -> bool;
}
```

Requirements:

- Thread-safe
- Time-based refill
- No external crates

### What This Tests

- Time handling
- Interior mutability
- Concurrency safety
- API design

---

# 🔴 Level 4 — Senior: Architecture & System Design

## 7️⃣ In-Memory Key-Value Store (Mini Redis)

### Problem

Build a TCP-based key-value store.

Supported commands:

```
SET key value
GET key
DEL key
EXPIRE key seconds
```

Requirements:

- Concurrent clients
- TTL support
- Thread-safe store
- Clean protocol parsing
- Graceful shutdown

### What This Tests

- Networking (`TcpListener`)
- Concurrency design
- Data modeling
- Lifetime management
- Production-style error handling

Bonus:

- Use async runtime (e.g., Tokio)
- Implement persistence to disk

---

## 8️⃣ Concurrent Web Crawler

### Problem

Build a concurrent crawler:

- Input: starting URL
- Crawl pages within same domain
- Limit concurrency
- Avoid duplicate visits

Requirements:

- Worker pool
- Shared visited set
- Timeout handling
- Graceful shutdown

### What This Tests

- Architecture
- Thread coordination
- HashSet sharing
- Design decisions

---

# 🧠 Senior Bonus: Tradeoff Discussion Round

After implementation, ask candidate:

1. Where does Rust help prevent bugs here?
2. Where did the borrow checker slow you down?
3. How would this scale to 1M req/sec?
4. What would change in a distributed system?
5. How would you add observability?

This reveals:

- Depth of systems thinking
- Experience with production systems
- Architectural maturity

---

# 🏁 Evaluation Matrix

| Level        | Focus           | What You Evaluate      |
| ------------ | --------------- | ---------------------- |
| Beginner     | Control flow    | Logical correctness    |
| Intermediate | Data structures | Ownership & memory     |
| Advanced     | Concurrency     | Safety & race handling |
| Senior       | Architecture    | Design & tradeoffs     |

---

# 🔥 If You Want to Make It Even Harder

Add:

- Implement a simplified version of the Rust ownership system in another language.
- Design a lock-free queue.
- Implement consistent hashing.
- Build a mini async runtime.

---

If you'd like, I can also:

- Provide reference implementations
- Create an interview rubric
- Convert this into a take-home assignment
- Create a 2-hour live coding interview structure
- Make a real-world FAANG-style Rust systems interview

Let me know your target audience (startup vs big tech vs systems team), and I’ll tailor it accordingly.
