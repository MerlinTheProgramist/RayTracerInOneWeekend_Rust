# Raytracing In One Weekend (in Rust + some other bits)
[original publication](https://raytracing.github.io/)

## How to build & Run:
```bash
cargo build -r # build with release
RUST_LOG=info ./target/release/rust # run the release with logging 
```

Challanges: 
  - Implement other planar shapes based on the notes in hittable/quad.rs, 
    like traingles, disks, rings, bitmap shapes or a Mandelbrot shape!