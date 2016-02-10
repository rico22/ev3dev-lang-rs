//! ev3dev_lang_rs -- Rust language bindings for ev3dev.org systems
//!
//! The ev3dev_lang_rs crate provides an ev3dev-lang standards-conforming API
//! for access to sensors and actuators of EV3 and similar robot systems.
//!
//~autogen autogen-version


//! Sections of this code were auto-generated based on spec v1.0.0.


//~autogen

// #![feature(plugin)]

// #![plugin(clippy)]

/// The system module provides a basis for specializing the file system for
/// testing.
pub mod system;

/// The testbase module provides helpers for unit testing.
#[macro_use]
pub mod testbase;

/// The device module provides a standards-conforming Device struct, with
/// associated types and traits.
pub mod device;

/// The sensor modules provides standards-conforming Sensor APIs.
pub mod sensor;

