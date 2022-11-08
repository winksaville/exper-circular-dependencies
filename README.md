# Experiment with circular dependencies

# Run

```
wink@3900x 22-11-08T03:28:54.621Z:~/prgs/rust/myrepos/exper-circular-dependencies (main)
$ cargo run
   Compiling exper-circular-dependencies v0.1.0 (/home/wink/prgs/rust/myrepos/exper-circular-dependencies)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/exper-circular-dependencies`
s1.strong_count=1 s2.strong_count=1
s1.weak_count=0 s2.weak_count=0
s1.strong_count=2 s2.strong_count=2
s1.weak_count=0 s2.weak_count=0
v1=0 v2=0
s1.v1=1 s2.v2=-1
v1=1 v2=-1
s1.v1=2 s2.v2=-2
v1=2 v2=-2
s1.strong_count=2 s2.strong_count=2
s1.weak_count=0 s2.weak_count=0
```

# Test

```
wink@3900x 22-11-08T03:30:01.647Z:~/prgs/rust/myrepos/exper-circular-dependencies (main)
$ cargo test
   Compiling exper-circular-dependencies v0.1.0 (/home/wink/prgs/rust/myrepos/exper-circular-dependencies)
    Finished test [unoptimized + debuginfo] target(s) in 0.18s
     Running unittests src/main.rs (target/debug/deps/exper_circular_dependencies-dc1c25c896ad41e1)

running 1 test
test test::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
