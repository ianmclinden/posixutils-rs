//
// Copyright (c) 2024 Jeff Garzik
//
// This file is part of the posixutils-rs project covered under
// the MIT License.  For the full license text, please see the LICENSE
// file in the root directory of this project.
// SPDX-License-Identifier: MIT
//

pub mod group;
pub mod io;
pub mod modestr;
pub mod testing;
pub mod utmpx;
pub mod lzw;

pub const PROJECT_NAME: &'static str = "posixutils-rs";

pub const BUFSZ: usize = 8 * 1024;

pub use testing::*;
