# 100 Elaborative Rust Interview Questions

1. How can you take a `Vec<i32>` and return a `Vec<i32>` containing only the squares of the even numbers, using iterator methods (`iter`, `filter`, `map`, `collect`) without using explicit loops? Explain the advantages of using iterator combinators over traditional loops in terms of performance and readability.
2. What are lifetimes in Rust? Write a function `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str` with explicit lifetime annotations and explain why lifetime elision does not apply here. Discuss how lifetimes help prevent dangling references and memory safety issues.
3. How do you implement the `From` and `Into` traits for a newtype `UserId(u64)`? Demonstrate converting between `u64` and `UserId` using `.into()`. Why are these traits important for ergonomic type conversions in Rust?
4. Describe how to design a configuration struct with optional fields using the builder pattern in Rust. How would you implement `Default` and chain setters that return `Self`? What are the benefits of using the builder pattern for complex configuration objects?
5. How do you handle errors in Rust using `Result`, the `?` operator, and the `thiserror` crate? Write a function `read_config(path) -> Result<Config, ConfigError>` and define a custom error type. Explain how error propagation works and why custom error types are useful.
6. How can you compute a moving average over a `Vec<f64>` using iterator adapters like `windows` and `map`? Return the result as a `Vec<f64>`. Discuss the efficiency of this approach compared to manual iteration.
7. Define an enum `Shape` and implement an `area()` method. How would you use the `matches!` macro to count only `Circle` variants in a slice? Explain the use of pattern matching and macros for concise code.
8. How do you build a word frequency counter for a `&str` using `split_whitespace` and `HashMap`'s `entry().or_insert(0) += 1` pattern? Discuss the role of entry API in efficient hashmap updates.
9. Write a function to compute the factorial of `n` using `Iterator::fold` over the range `(1..=n)`. Explain why this approach is idiomatic in Rust and how it leverages functional programming concepts.
10. How do you implement a function `fn middle<T>(slice: &[T]) -> Option<&T>` that returns the middle element of a slice? Test your function with both strings and integers. Discuss how Rust's slice API helps with safe indexing.
11. What are generic constraints in Rust? Write a function `fn print_both<T: std::fmt::Display + std::fmt::Debug>(t: &T)` that prints formatted output. Why are trait bounds important for generic programming?
12. Explain associated types in traits. Define a trait `Storage` with associated types `Key` and `Value`, and implement it for a `HashMapStorage`. How do associated types improve trait flexibility?
13. What is `PhantomData` and how is it used? Demonstrate with a `TypedId<T>(u64, PhantomData<T>)` to create distinct types like `UserId` and `ProductId` that cannot be intermixed. Why is this pattern useful for type safety?
14. How do you overload operators in Rust? Implement `Add` for a `Point` struct so that `p1 + p2` works, and explain the required trait and associated type. What are the implications for custom types?
15. Describe the drop order and RAII in Rust. Implement a `TempDir` that creates a temp folder in `new()` and removes it in `Drop`. How can you test drop order with print statements? Why is RAII important for resource management?
16. How do you build a tree structure using `Rc<RefCell<T>>` for nodes with children? Implement an `add_child` method that updates the parent via `Weak`. Discuss the trade-offs of using reference counting and interior mutability.
17. Implement a wrapper type `MyBox<T>` that logs accesses to `deref` and `deref_mut`. How do you implement the `Deref` and `DerefMut` traits? What are the use cases for custom smart pointers?
18. How do you create a custom iterator in Rust? Implement a `Counter` with `new(start, step)` that yields successive values and implements `Iterator` for `u64`. Why is implementing custom iterators useful?
19. How can you flatten a `Vec<Vec<i32>>` and filter its elements using iterator chaining (`into_iter().flatten().filter().collect()`)? Discuss the power of iterator combinators for data transformation.
20. What are pattern matching ergonomics in Rust? Show how to match on `&Option<String>` to avoid cloning, using `ref` and `ref mut` bindings. Why is ergonomic pattern matching important for performance?
21. How do you implement a producer/consumer pattern using bounded channels (`sync_channel`) in Rust? Spawn threads and ensure graceful shutdown by dropping the sender. What are the challenges of concurrent programming in Rust?
22. How do you use `Arc<Mutex<_>>` to share a counter incremented by multiple threads? Use `thread::scope` to avoid `'static` bounds. Explain the difference between `Arc` and `Rc` in multithreaded contexts.
23. How do you implement a read-mostly cache using `RwLock<HashMap<K,V>>`? Measure lock time with `Instant` and explain lock contention. What are the benefits and drawbacks of read-write locks?
24. Compare parallel mapping over a slice using crossbeam scoped threads versus using Rayon for large data sets. What are the trade-offs in terms of ergonomics and performance?
25. How do you use Rayon parallel iterators to compute a parallel sum and histogram? Ensure no data races by using `reduce_with`. Why is Rayon preferred for data parallelism?
26. What is the purpose of the `#[tokio::main]` attribute? Write an async main function that fetches two URLs concurrently using `join!`. How does async/await improve concurrency?
27. How do you use `tokio::time::timeout` for async requests? Handle `Elapsed` errors and cancel a background task by dropping its handle. Why is timeout handling critical in network programming?
28. How do you implement backpressure in async Rust using `tokio::sync::mpsc` bounded channels? Why do producers need to `await send()`? Discuss the importance of backpressure in scalable systems.
29. How do you expose a `tokio::sync::mpsc` channel as a `Stream` using `tokio_stream::wrappers::ReceiverStream`? Consume the stream with `while let Some(x)`. What are the benefits of using streams in async Rust?
30. How do you use `tokio::select!` to race a TCP read against a timeout, and handle whichever completes first? Why is select useful for multiplexing async events?
31. How do you read a large file asynchronously using `tokio::fs` and compute its SHA-256 incrementally with `tokio::io::BufReader`? Discuss the challenges of async file I/O.
32. How do you implement exponential backoff and jitter for retries in async Rust? Write a function `retry(f, attempts, base_delay)`. Why is backoff important for reliability?
33. How do you define async trait objects using the `async-trait` crate? Implement a `Repository` trait with `async fn get(id)->Result<Item>` and provide an implementation. What are the limitations of async traits in Rust?
34. How do you build a minimal Axum API with a GET `/health` endpoint and a POST `/items` endpoint that accepts JSON and returns the created item? Include extractors and state management. Why is Axum a good choice for web APIs?
35. How do you define an enum with an internally tagged representation using Serde? Serialize and deserialize sample JSON. What are the advantages of tagged enums?
36. How do you use `thiserror` for precise library errors and `anyhow` for error bubbling in a binary? Show how to add context to errors. Why is error context important?
37. How do you add spans and instrumentation to async handlers using the `tracing` crate? Include span fields and use the `tracing-subscriber` fmt layer. How does tracing help with observability?
38. How do you use Cargo feature flags to control compile-time behavior? Implement `parse_config()` that supports JSON or YAML via `cfg`. What are the best practices for feature flags?
39. How do you create a Cargo workspace with a library crate and a binary crate that uses the library? Demonstrate path dependencies. Why are workspaces useful for large projects?
40. How do you use the `itertools` crate's `group_by` to group consecutive equal items in an iterator, avoiding manual state management? What are the benefits of using external crates for iterator utilities?
41. How do you create a proc-macro crate with a custom derive macro (e.g., `derive(Builder)`) that generates a builder for a struct? Outline the steps. What are the challenges of writing proc macros?
42. How do you write a declarative macro (TT-muncher) like `hashmap! { k => v, ... }` that builds a `HashMap` and supports trailing commas? Why is macro flexibility important?
43. What is macro hygiene in Rust? Write a `log_expr!` macro that evaluates an expression once and logs its value, using `let` bindings for hygiene. How does hygiene prevent bugs?
44. How do you use `const fn` and const generics to implement a fixed-size matrix type `FixedMatrix<T, const N: usize>`? Implement an `identity()` method as a `const fn`. Why are const generics powerful?
45. How do you create zero-cost newtypes for units like `Meters(f64)` and `Seconds(f64)`? Implement `Div` to yield `Speed` and prevent mixing using a `PhantomData` marker. How does this pattern prevent unit errors?
46. What is the typestate pattern? Implement an `HttpRequestBuilder` that prevents `send()` until both URL and method are set, encoding state via generic type parameters. Why is typestate useful for API safety?
47. How do you implement the sealed trait pattern in Rust? Create a trait in a module with a private `Sealed` type to prevent external implementations, and explain the rationale. What problems does sealing solve?
48. What are higher-ranked trait bounds (HRTB)? Write a function `fn apply<'a, F>(f: F) where F: for<'b> Fn(&'b str) -> &'b str` and demonstrate its use. Why are HRTBs needed?
49. What are generic associated types (GATs)? Define a trait `StreamingIterator` with `type Item<'a>`, and implement it for a struct over a buffer. How do GATs improve trait expressiveness?
50. How do you explore variance in Rust using `PhantomData`? Show examples of invariance and covariance, and explain why `&T` is covariant while `Cell<T>` is invariant. Why does variance matter?
51. What are auto traits like `Send` and `Sync`? Build a type that is `!Send` due to a raw pointer, then fix it by wrapping in `Arc<Mutex<>>` and explain the results. How do auto traits affect concurrency?
52. How do you use `Any` and `downcast` in Rust? Create a heterogeneous `Vec<Box<dyn Any>>`, push different types, and retrieve them by type using `downcast_ref`. What are the use cases for type erasure?
53. What are the basics of `unsafe` in Rust? Create and dereference raw pointers, demonstrate undefined behavior risks, and show correct usage within an `unsafe` block. Why is `unsafe` needed?
54. How do you call C functions from Rust using FFI? Declare an `extern "C"` function to call `puts` from libc, link, and call it safely. What are the safety concerns with FFI?
55. How do you call Rust functions from C using FFI? Declare a `#[no_mangle] extern "C" fn add(a: i32, b: i32) -> i32`, build a staticlib, and show a C snippet that calls it. How do you ensure ABI compatibility?
56. How do you write a safe wrapper for a C function that returns a pointer and length? Ensure lifetime correctness in your Rust API. What are the risks of incorrect lifetimes?
57. What is pinning in Rust? Implement a self-referential struct using `Pin<Box<T>>` with a field holding a pointer into itself, and explain why pinning is needed. How does pinning prevent memory movement?
58. How do you use uninitialized memory safely in Rust? Use `MaybeUninit<[u8; 1024]>` to fill a buffer from a read operation, avoid UB, and then call `assume_init()`. What are the dangers of uninitialized memory?
59. How do you implement a lock-free concurrent counter using `AtomicU64` and `fetch_add`? Discuss memory ordering (`SeqCst` vs `Relaxed`). Why is lock-free programming challenging?
60. How do you implement a lock-free stack (Treiber stack) using `AtomicPtr`? Ensure safe `Drop` on pop operations. What are the pitfalls of manual memory management?
61. How do you replace `std::sync` locks with `parking_lot` locks in Rust? Benchmark the difference using the `criterion` crate. Why might you prefer `parking_lot`?
62. How do you benchmark different factorial implementations (naive vs `iter::fold`) using `criterion`? Output a report and avoid misleading results from `--release`. What are best practices for benchmarking?
63. How do you use `proptest` for property-based testing in Rust? Test a reversible encode/decode pair and shrink failing cases. Why is property-based testing valuable?
64. How do you verify algebraic laws (e.g., Monoid laws) for a custom type using QuickCheck-style property tests? What are the benefits of law-based testing?
65. How do you serialize a struct to multiple formats (JSON, YAML, CBOR) in Rust? Compare sizes and use `serde(default)` for missing fields. Why is multi-format serialization useful?
66. How do you walk the filesystem using `walkdir` to compute blake3 hashes of files concurrently? Show the first 8 hex characters of each hash. What are the challenges of concurrent file I/O?
67. How do you read from stdin line-by-line using `BufRead::lines`, transform the input, and write to stdout efficiently? Why is streaming I/O important?
68. How do you profile memory usage in Rust? Use `mem::size_of::<T>()` and `size_of_val` for nested structs, and explain padding and alignment. How does memory layout affect performance?
69. How do you use `SmallVec` or `ArrayVec` to collect small results without heap allocation for up to N items? What are the trade-offs of stack vs heap allocation?
70. How do you build an LRU cache using a `HashMap` and a doubly linked list (`Rc<RefCell<Node>>` or raw pointers with `unsafe` for performance)? Why is LRU caching important?
71. How do you implement a fixed-capacity MPMC ring buffer using atomics? When would you fall back to a `Mutex` if stuck? What are the challenges of lock-free buffers?
72. How do you implement a custom allocator in Rust? Outline how to implement `GlobalAlloc` for a bump allocator in a toy `no_std` example. Why might you need a custom allocator?
73. How do you build a crate with `#![no_std]`, use only the `core` library, and expose a function usable in embedded environments? How do you unit test with `std` via `cfg`? What are the constraints of `no_std`?
74. How do you implement an embedded-style fixed-point decimal type using an `i64` backing? Implement arithmetic operations with scaling. Why is fixed-point arithmetic used in embedded systems?
75. How do you build a tiny WebAssembly function using `wasm_bindgen` that reverses a string? Include comments on how to use it from JavaScript. What are the benefits of WASM for Rust?
76. How do you use SIMD in Rust to sum `f32` slices with SSE/AVX when available, and fallback to scalar code? Use `cfg(target_feature)` for conditional compilation. Why is SIMD important for performance?
77. How do you implement a non-blocking parser as a state machine enum in Rust? Advance the state with `advance(&mut self, byte) -> Option<Token>`. What are the advantages of state machines?
78. How do you implement a Base64 encoder/decoder using iterator adapters in Rust? Test your implementation against the `base64` crate for correctness. Why is iterator-based encoding efficient?
79. How do you write a simple glob matcher supporting `*` and `?` using dynamic programming, without using the `regex` crate? What are the limitations of glob matching?
80. How do you build an HTTP client with `reqwest` to download a file with a streaming body? Show progress using bytes read and total length. Why is streaming important for large downloads?
81. How do you use `tokio_util::sync::CancellationToken` to cancel a long-running async task on Ctrl+C? What are the best practices for cancellation in async Rust?
82. How do you build a bounded async queue with backpressure in Rust? Measure queue depth over time as producers and consumers operate at different speeds. Why is backpressure critical for system stability?
83. How do you implement a toy actor model with a single-threaded executor and a mailbox per actor? Implement ping-pong messages between actors. What are the benefits of the actor model?
84. How do you implement a basic work-stealing thread pool using `crossbeam-deque`? Run tasks with `futures::executor::block_on` via a `spawn_blocking` equivalent. Why is work-stealing useful?
85. How do you implement a mini-Redis server that parses RESP and supports a subset of commands (`PING`, `GET`, `SET`) over TCP, handling concurrency? What are the challenges of building network servers?
86. How do you build a CLI with `clap` using derived `Args` and `Subcommand`? Add a `--verbose` flag to change the tracing level. Why is CLI ergonomics important?
87. How do you layer configuration loading in Rust (default, file, env, CLI args)? Merge with precedence using `serde`, `envy`, and `clap`. What are the challenges of config layering?
88. How do you implement HS256 JWT signing and verification for a minimal web API? Manage claims like `exp`, `iat`, and `nbf`. Why is JWT widely used for authentication?
89. How do you use `sqlx` to connect to Postgres, run a migration, and implement CRUD for a `Todo` model with transactions? What are the benefits of using SQLx?
90. How do you use SERIALIZABLE isolation and automatic retries on serialization failure in database transactions? Why is transaction isolation important?
91. How do you add a Redis caching layer with TTL in Rust? Mitigate cache stampede with random jitter. What are the best practices for caching?
92. How do you instrument HTTP handlers and database calls with `tracing` and metrics (e.g., Prometheus exporter) for observability? Why is observability critical for production systems?
93. How do you structure your test pyramid in Rust? Write unit tests for pure logic, integration tests for HTTP endpoints (`reqwest`), and use `testcontainers` for Postgres. What is the value of a test pyramid?
94. How do you use `cargo fuzz` to fuzz your parser, fix discovered panics or UB, and add regression tests? Why is fuzzing important for security?
95. How do you ensure security in Rust by using constant-time equality for secrets, forbidding unsafe except in specific modules, and auditing dependencies? What are the best practices for secure Rust code?
96. How do you convert panicking code to return `Result` instead? Use `std::panic::catch_unwind` where appropriate and document invariants. Why is error handling preferred over panics?
97. How do you create a domain-specific error enum with display messages and source chaining? Map errors to HTTP status codes. Why is error taxonomy important?
98. How do you write documentation and examples for public APIs in Rust? Add doctests that compile and run, and include an `examples/` crate. What are the benefits of good documentation?
99. How do you prepare a crate for publishing? Set up `Cargo.toml` metadata, license, README, categories, keywords, and explain your semver strategy. Why is proper packaging important?
100. How do you profile performance in Rust using `flamegraph` or `pprof-rs`? Optimize hot paths, allocations, and copies based on profiling results. What are the best practices for performance profiling?
