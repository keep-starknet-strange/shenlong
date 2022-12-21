//! Shenlong is a library for building LLVM IR from Cairo.
//!
//! Specifically, it enables you to compile Sierra code to LLVM IR.
//!
//! Sierra is the intermediate representation of Cairo, ensuring that the resulting Cairo assembly is always valid and provable.
//!
//! The flow of the compilation is as follows:
//!
//! ```text
//!+-----------------------+
//!|                       |
//!|     Cairo code        |
//!|                       |
//!+----------+------------+
//!           |
//!           |
//!+----------+------------+
//!|                       |
//!|      Sierra           |
//!+----------+------------+
//!           |
//!           |
//!+----------v------------+
//!|       LLVM IR         |
//!|                       |
//!+-----------------------+
//! ```

/// The core library of Shenlong compiler.
pub mod core {
    pub use shenlong_core::*;
}
