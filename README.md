# raytracer

A ray tracer written in Rust.

## Why a ray tracer?

I have always been interested in computer graphics and had always wanted to
write a ray tracer since doing my Advanced Graphics course at university, so it
seemed a natural fit when trying to find a project to learn Rust with. In
addition ray tracing produces beautiful images rather than files full of numeric
values or unexciting csv data or the like.

## Implementation

I will be working primarily from [The Ray Tracer
Challenge](http://raytracerchallenge.com) using Rust as my language of choice.
Since the book provides only the general algorithm and test cases it will
provide an opportunity to use some of Rust's more interesting features and its
ease of refactoring to refine the design as time goes on. While I am mostly
intending to follow the book, a ray tracer provides ample opportunity for
extension and playing around with optimisations later on.

### Floating point calculations

For float calculations I will be using the
[float-cmp](https://crates.io/crates/float-cmp) crate to help with comparing
floating point numbers. It seems to offer the easiest to use API and has good
enough defaults for comparisons for our usage. Technically there isn't any good
enough default for epsilon / ulps values given the fun involved with computers
and floating point math, each calculation and algorithm should be looked at to
determine what are the best values, etc., etc. But for our usage we don't
particularly care about that level of granularity or precision.

Their macros have been reimplemented to avoid having to pass the type every time
we do a comparison.

## For the future

* Consider making things generic over other float types (f32, arbitrary precision
  floats, fixed decimals, etc.) and examine performance.
