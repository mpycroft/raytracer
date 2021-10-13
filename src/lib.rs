//! The main ray tracer code is split into a library containing nearly all the
//! code and the main binary. This provides a nice separation as well as making
//! it easier later on to do benchmarking and doc tests that have issues with
//! being in a binary.

pub mod math;
