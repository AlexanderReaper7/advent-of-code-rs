//! This crate is used to generate the auto_import.rs and mod.rs files for each year and day.

use quote::quote;
use std::{
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

/// Run this function to generate the auto_import.rs and mod.rs files for each year folder and day file.
/// 
/// # Example
/// 
/// ```rust
/// use aoc_auto::aoc_auto;
/// fn main() {
///    aoc_auto();
/// }
/// ```
pub fn aoc_auto() {
    // get years to make mod files for from src/, each folder is a year formatted as y20XX/
    let years: Vec<String> = Path::new("src/")
        .read_dir()
        .unwrap()
        .map(|e| e.unwrap())
        .filter(|e| e.path().is_dir() && e.file_name().to_str().unwrap().starts_with("y20"))
        .map(|e| e.file_name().to_str().unwrap().to_owned())
        .collect();
    // for each year, get days to make mod files for from src/y20XX/, each folder is a day formatted as dX.rs
    for year in &years {
        let days: Vec<String> = Path::new("src/")
            .join(year.clone())
            .read_dir()
            .unwrap()
            .map(|e| e.unwrap())
            .filter(|e| {
                let filename = e.file_name().into_string().unwrap();
                e.path().is_file() && filename.starts_with("d") && filename.ends_with(".rs")
            })
            .map(|e| e.file_name().to_str().unwrap().to_owned())
            .collect();

        // TODO: move parts to separate function in days instead of year module

        let days_expr: Vec<syn::Expr> = days
            .iter()
            .map(|e| {
                let d = e.replace(".rs", "");
                syn::parse_str::<syn::Expr>(&d).unwrap()
            })
            .collect();
        let days_num_expr: Vec<syn::Expr> = days
            .iter()
            .map(|e| e.replace("d", "").replace(".rs", ""))
            .map(|e| syn::parse_str::<syn::Expr>(&e).unwrap())
            .collect();
        let mod_code = quote! {
            //! Auto-generated file by build script, do not edit!
            #(pub mod #days_expr;)*

            /// Selects the function for the given day and part
            pub fn select_function(day: u32, part: u32) -> fn(String) -> String {
                match day {
                    #(#days_num_expr =>
                        match part {
                            1 => #days_expr::part1,
                            2 => #days_expr::part2,
                            _ => panic!("Invalid part!"),
                        }
                    ),*
                    _ => panic!("Invalid day!"),
                }
            }
        };

        let mut mod_file_path = Path::new("src/").join(year).join("mod.rs");
        write_and_format(mod_code.to_string(), &mut mod_file_path);
    }

    let years_expr: Vec<syn::Expr> = years
        .iter()
        .map(|e| syn::parse_str::<syn::Expr>(&e).unwrap())
        .collect();
    let auto_import_file = Path::new("src/auto_import.rs").to_owned();
    let years_mod: Vec<String> = years.iter().map(|e| format!("{}/mod.rs", e)).collect();
    let years_num_expr: Vec<syn::Expr> = years
        .iter()
        .map(|e| e.replace("y", ""))
        .map(|e| syn::parse_str::<syn::Expr>(&e).unwrap())
        .collect();

    let auto_import_code = quote! {
        //! Auto-generated file by build script, do not edit!
        #(
            #[path = #years_mod]
            pub mod #years_expr;
        )*
        /// Selects the function for the given year, day, and part
        pub fn select_function(year: u32, day: u32, part: u32) -> fn(String) -> String {
            match year {
                #(#years_num_expr => #years_expr::select_function(day, part),)*
                _ => panic!("Invalid year!"),
            }
        }
    };

    write_and_format(auto_import_code.to_string(), &auto_import_file)
}

fn write_and_format(file: String, path: &PathBuf) {
    let syntax_tree = syn::parse_file(&file).unwrap();
    let text = prettyplease::unparse(&syntax_tree);
    let mut file: File = File::create(&path).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}
