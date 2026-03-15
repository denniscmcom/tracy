# Tracy

A ray tracer written in Rust. It supports rendering to a PPM file or in
real-time through an interactive render view.

![Sample scene](https://public.denniscm.com/repositories/tracy/sample-scene.png)

## Features

- Diffuse, metal, and dielectric materials
- Reflections and refractions
- Depth of field
- Anti-aliasing
- Real-time interactive render view
- PPM file output

## Build & Run

Requires Rust and Cargo.

### Real-time render view 

```
cargo run --release -- -render-view
```

### PPM file output 

```
cargo run --release
```

## References

Built following [Ray Tracing in One Weekend](https://raytracing.github.io),
with additional work on the real-time rendering mode.

