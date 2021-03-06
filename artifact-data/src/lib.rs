/*  artifact: the requirements tracking tool made for developers
 * Copyright (C) 2017  Garrett Berg <@vitiral, vitiral@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the Lesser GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the Lesser GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */
//! The artifact data crate defines the method of serializing
//! and deserializing raw artifact and processing them into
//! a full project.
//!
//! Note that almost all tests for artifact-data are in artifact-test

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(doc_markdown)]

extern crate base64;
extern crate ergo;
#[macro_use]
extern crate expect_macro;
extern crate failure;
#[macro_use]
extern crate matches;
extern crate petgraph;
extern crate rayon;
extern crate siphasher;

#[macro_use]
extern crate failure_derive;
extern crate time;

#[macro_use]
extern crate artifact_lib;
#[macro_use]
extern crate log;

// MODULES

pub mod artifact;
mod dev_prelude;
mod modify;
pub mod graph;
pub mod implemented;
mod intermediate;
mod project;
pub mod raw;
#[macro_use]
pub mod raw_names;
mod settings;

#[cfg(test)]
#[macro_use]
extern crate proptest;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

// #[cfg(test)]
// extern crate rand;

#[cfg(test)]
extern crate regex_generate;

#[cfg(test)]
extern crate tempdir;

#[cfg(test)]
extern crate unicode_segmentation;

pub use modify::modify_project;
pub use settings::{ART_DIR, SETTINGS_FILE};
pub use project::read_project;
