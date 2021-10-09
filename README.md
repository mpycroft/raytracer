# raytracer

A ray tracer written in Rust.

## Why a Ray Tracer?

I have always been interested in computer graphics and had always wanted to
write a ray tracer since doing my Advanced Graphics course at university, so it
seemed a natural fit when trying to find a project to learn Rust with. In
addition ray tracing produces beautiful images rather than files full of numeric
values or unexciting csv data or the like.

## Implementation

I will be working primarily from [The Ray Tracer
Challenge](http://raytracerchallenge.com). I had initially started a similar
project based on [Ray Tracing in One Weekend](https://raytracing.github.io) but
ended up running myself in rather unproductive circles.

Mostly these were self inflicted problems; since Rust can be as fast as C/C++ I
kept trying to make sure my code was at least as optimised as that of the RTiOW
example and spending an inordinate amount of time doing so. In addition the
nature of RTiOW being C++ based meant many of the design decisions weren't ideal
for Rust and while I did make appropriate changes, I still felt compelled to
keep fairly similar structure in order to make sure performance was similar.
Finally the limited nature of the RTiOW code (primarily because it was intended
to be fairly simple to understand) was sometimes frustrating e.g. the use of
trig rotations rather than using matrices, only using spheres as objects, no
scene files, etc. All of these left plenty of room for my own expansion of
course but combined ended up being a somewhat frustrating experience.

That said, I will be taking the lessons I learned from working through RTiOW
with me while working on tRTC, additionally I ended up adding a lot of my own
testing just to make sure I didn't mess up anything when refactoring so tRTC
being test driven will be a nice change. I do hope to eventually add some
information from [Physically Based Rendering](https://pbr-book.org) as interest
takes me but that book is far too mathematically heavy for me to use as an
initial basis for a hobby project.

### Tuples

Rather than following the book's usage of tuples as an underlying data structure
for both points and vectors we will implement them separately and use the type
system in Rust to our advantage. This will likely add some extra issues when we
get to matrices and the like but there is no good way to do this without
(ab)using dereferencing, having awkward access (i.e. using getters/setters) or
until some sort of fields in traits feature gets implemented. Since the only
real usage of homogenous coordinates (w elements) on points and vectors is to
make the matrix multiplication fall out and we already have that information in
the type we shouldn't hit any fundamental issues.
