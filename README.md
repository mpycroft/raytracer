# raytracer

[![Build](https://github.com/mpycroft/raytracer/actions/workflows/rust.yaml/badge.svg)](https://github.com/mpycroft/raytracer/actions)
[![Coverage](https://codecov.io/gh/mpycroft/raytracer/graph/badge.svg)](<https://codecov.io/gh/mpycroft/raytracer>)

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

### Groups

Rather than following the books graph structure for groups where children hold a
reference to their parents, we use a different approach where when each group is
constructed it "pushes down" its transformation to all its children. Primarily
this was done because having a parent pointer in each child Object would lead to
very verbose and complex lifetime annotation. It should be possible to do
however as there are no cycles in the graph.

Our approach makes things simpler and technically should provide a speed boost
as the matrix is precomputed in the final shape rather than having to perform
multiple matrix multiplications every time we intersect. It does however prove a
bit annoying when making sure everything is updated when creating each object.
Note that this design decision means we cannot add children to a group after its
creation; we need to create all child objects beforehand. This is however not a
great issue as this conforms with how we have been treating all object creation
so far.

## For the future

* ~~Consider making things generic over other float types (f32, arbitrary precision
  floats, fixed decimals, etc.) and examine performance.~~
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
* ~~Look at algorithms for splitting the objects in a scene into groups
  automatically to allow bounding box optimisations.~~

## Performance

### Matrices

Storing the inverted matrix with each object produces a significant speed up. On
an image with 3 spheres, 2 planes and 2 lights.

On a debug build rendering at 1,000x500 this change alone makes using the debug
build viable for testing.

* Don't store inverse: 188.65s
* Store inverse: 14.85s

On a release build rendering at 3,000x1,500.

* Don't store inverse: 14.86s
* Store inverse: 3.72s

Storing the inverted transpose matrix didn't have any notable speedup so we will
continue to calculate that as needed.

While doing the above I missed an inversion in Camera. Fixing this to also store
the inverted transformation cuts debug builds down to 2.71s and release builds
to 2.37s (at 3x the pixels of the debug build as above).

### Rayon

Using [rayon](https://crates.io/crates/rayon) provides a significant speedup
when rendering large and complex images. Rendering the Chapter11 image, which
contains many transparent and reflective surfaces at a size of 10,000 x 5,000
with a recursion depth of 30:

* Single threaded: 264.26s
* Rayon: 71.55s

### Bounding Boxes

There is a significant speedup when using the bounding box checks on groups of
objects. When rendering the Chapter14Spheres image at 3,000 x 2,400, depth of
20, single threaded with a random seed of 0, which contains multiple groups each
contains a number of different spheres, we see the following:

* Without bounds checking: 20.31s
* With bounds checking: 6.33s

### Bounding Volume Hierarchies

In addition to the general bounding box improvements, once we implemented
(naive) splitting of children of groups we see a significant performance
improvement. For the Chapter15 scene, running single threaded:

* Without dividing: 87.64s
* Dividing into groups of 50: 9.90s

For the BoundingBox scene running multi threaded:

* Without dividing: 188.23s
* Dividing into groups of 50: 3.07s

A value of 50 was picked arbitrarily, no significant difference is shown for
values from 1 to a 100 or so, values 200, 500, etc. do start to show less
improvement however.

### F32 vs F64

There doesn't appear to be any significant difference between using 32 and 64
bit floats for our use case. Under half a second (for a 15s run) which is well
within the noise level. This would probably change if we want to look at SIMD
instructions since generally we pack 4 f32s into a single instruction.
