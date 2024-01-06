# raytracer

A ray tracer written in Rust.

## Why a ray tracer?

I have always been interested in computer graphics and had always wanted to
write a ray tracer since doing my Advanced Graphics course at university, so it
seemed a natural fit when trying to find a project to learn Rust with. In
addition ray tracing produces beautiful images rather than files full of numeric
values or unexciting csv data or the like.

## Images

Scene renders can be seen [here](images/README.md).

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

### Tuples

Rather than following the book's usage of tuples as an underlying data structure
for both points and vectors we will implement them separately and use the type
system in Rust to our advantage. This will allow us to not need to do some sorts
of checks and we can enforce only valid operations on vectors and points. Since
the only real usage of homogenous coordinates (w elements) on points and vectors
is to make the matrix multiplication fall out and we already have that
information in the type we shouldn't hit any fundamental issues.

## For the future

* Consider making things generic over other float types (f32, arbitrary precision
  floats, fixed decimals, etc.) and examine performance.
* We may need to revisit our use of 3 element points and vectors if we ever look
  into SIMD style operations where we would actually want x, y, z, w elements
  for speed.
* Matrices are currently Copy for convenience but probably shouldn't be since
  they are generally 16 f64, however removing copy makes them a bit more
  annoying to use (e.g. implementing MulAssign, checks have to be a references,
  etc.). If we find that we spend a lot of time copying we may want to revisit
  this later.
* Intersections are not sorted when added to the List and we just search the
  vector of values for a minimum using iterators. May need to check later on if
  its more efficient to use a different data structure or store elements sorted,
  etc. when we have lots of hits.
* ~~We only store the transformation matrix with each object, we may consider
  precomputing the inverted matrix as well.~~

## Performance

### Matrices

Storing the inverted matrix with each object produces a significant speed up. On
an image with 3 spheres, 2 planes and 2 lights.

On a debug build rendering at 1000x500 this change alone makes using the debug
build viable for testing.

* Don't store inverse: 188.65s
* Store inverse: 14.85s

On a release build rendering at 3000x1500.

* Don't store inverse: 14.86s
* Store inverse: 3.72s

Storing the inverted transpose matrix didn't have any notable speedup so we will
continue to calculate that as needed.

While doing the above I missed an inversion in Camera. Fixing this to also store
the inverted transformation cuts debug builds down to 2.71s and release builds
to 2.37s (at 3x the pixels of the debug build as above).
