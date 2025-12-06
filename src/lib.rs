// Enable clippy lints.
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::allow_attributes)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::impl_trait_in_params)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::mod_module_files)]
#![warn(clippy::panic)]
#![warn(clippy::redundant_type_annotations)]
#![warn(clippy::renamed_function_params)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::same_name_method)]
#![warn(clippy::semicolon_outside_block)]
#![warn(clippy::single_char_lifetime_names)]
#![warn(clippy::str_to_string)]
#![warn(clippy::tests_outside_test_module)]
#![warn(clippy::try_err)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unused_trait_names)]
#![warn(clippy::use_debug)]
#![warn(clippy::verbose_file_reads)]
// Suppressions.
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::unused_self)]

pub mod graph;
pub mod hir;
pub mod hir_to_mir;
pub mod lexer;
pub mod mir;
pub mod mir_passes;
pub mod ops;
pub mod parser;
pub mod position;
pub mod scope;
pub mod token;
pub mod type_resolver;
