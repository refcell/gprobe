#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]
#![allow(deprecated)]

//! # GProbe
//!
//! GProbe is a tool for probing the data source and generating a schema for it.
//!
//! ## Usage
//!
//! // TODO:

/// Cli Args
pub mod cli;

/// Telemetry
pub mod telemetry;

/// The core [crate::cli::GProbe] runner
pub mod runner;