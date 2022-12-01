use std::{io::prelude::*, path::{Path, PathBuf}, fs::File, process::Command};
use quote::quote;

fn main () {
    quoted_generate_auto_import();
}

#[deprecated(since = "0.1.0", note = "Use `quoted_generate_auto_import` instead")]
fn naive_generate_auto_import() {
    // get year folders
    let years: Vec<PathBuf> = Path::new("src/")
    .read_dir().unwrap()
    .map(|e| e.unwrap())
    .filter(|e| e.path().is_dir() && e.file_name().to_str().unwrap().starts_with("y20"))
    .map(|e| e.path())
    .collect();

    // create year modules
    for year in &years {
        // get day files
        let days: Vec<PathBuf> = year
        .read_dir().unwrap()
        .map(|e| e.unwrap())
        .filter(|e| e.path().is_file() && e.file_name().to_str().unwrap().starts_with("d") && e.file_name().to_str().unwrap().ends_with(".rs"))
        .map(|e| e.path())
        .collect();

        // create mod.rs
        let mut mod_file = File::create(year.join("mod.rs")).unwrap();
        writeln!(mod_file, "//! Auto-generated file by build script, do not edit!").unwrap();
        // import all days
        for day in &days {
            let day_name = day.file_name().unwrap().to_str().unwrap().replace(".rs", "");
            writeln!(mod_file, "pub mod {};", day_name).unwrap();
        }
        // create selection function
        writeln!(mod_file, "\npub fn select_function(day: u32, part: u32) -> fn(String) -> String {{\n\tmatch day {{").unwrap();
        for day in &days {
            let day_name = day.file_name().unwrap().to_str().unwrap().replace(".rs", "");
            let day_number = day_name.replace("d", ""); 
            writeln!(mod_file, "\t\t{} => match part {{", day_number).unwrap();
            for part in 1..=2 { // TODO: make this dynamic
                writeln!(mod_file, "\t\t\t{} => {}::part{},", part, day_name, part).unwrap();
            }
            writeln!(mod_file, "\t\t\t_ => panic!(\"Invalid part!\"),\n\t\t}},").unwrap();
        }
        writeln!(mod_file, "\t\t_ => panic!(\"Invalid day!\"),\n\t}}\n}}").unwrap();

        println!("cargo:rerun-if-changed={}", year.display());
    }

    // create auto_import.rs
    let mut auto_import_file = File::create("src/auto_import.rs").unwrap();
    writeln!(auto_import_file, "//! Auto-generated file by build script, do not edit!").unwrap();
    // import all years
    for year in &years {
        let year_name = year.file_name().unwrap().to_str().unwrap();
        writeln!(auto_import_file, "#[path = \"{year_name}/mod.rs\"]\npub mod {year_name};", year_name = year_name).unwrap();
    }
    // create selection function
    writeln!(auto_import_file, "\npub fn select_function(year: u32, day: u32, part: u32) -> fn(String) -> String {{\n\tmatch year {{").unwrap();
    for year in &years {
        let year_name = year.file_name().unwrap().to_str().unwrap();
        writeln!(auto_import_file, "\t\t{} => {}::select_function(day, part),", year_name.replace("y", ""), year_name).unwrap();
    }
    writeln!(auto_import_file, "\t\t_ => panic!(\"Invalid year!\"),\n\t}}\n}}").unwrap();

    println!("cargo:rerun-if-changed=src/auto_import.rs");
    println!("cargo:rerun-if-changed=build.rs");
}

fn quoted_generate_auto_import() {

    let years: Vec<String> = Path::new("src/")
    .read_dir().unwrap()
    .map(|e| e.unwrap())
    .filter(|e| e.path().is_dir() && e.file_name().to_str().unwrap().starts_with("y20"))
    .map(|e| e.file_name().to_str().unwrap().to_owned())
    .collect();

    for year in &years {
        let days: Vec<String> = Path::new("src/").join(year.clone()).read_dir().unwrap()
        .map(|e| e.unwrap())
        .filter(|e| e.path().is_file() && e.file_name().to_str().unwrap().starts_with("d") && e.file_name().to_str().unwrap().ends_with(".rs"))
        .map(|e| e.file_name().to_str().unwrap().to_owned())
        .collect();

        // TODO: move parts to separate function in days instead of year module

        let days_expr: Vec<syn::Expr> = days.iter().map(|e| {let d = e.replace(".rs", ""); syn::parse_str::<syn::Expr>(&d).unwrap()}).collect();
        let days_num_expr: Vec<syn::Expr> = days.iter().map(|e| e.replace("d", "").replace(".rs", "")).map(|e| syn::parse_str::<syn::Expr>(&e).unwrap()).collect();
        let mod_code = quote! {
            //! Auto-generated file by build script, do not edit!
            #(pub mod #days_expr;)*
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

        let mut mod_file = Path::new("src/").join(year).join("mod.rs");
        write_and_format(mod_code.to_string(), &mut mod_file);
    }

    let years_expr: Vec<syn::Expr> = years.iter().map(|e| syn::parse_str::<syn::Expr>(&e).unwrap()).collect();
    let auto_import_file = Path::new("src/auto_import.rs").to_owned();
    let years_mod: Vec<String> = years.iter().map(|e| format!("{}/mod.rs", e)).collect();
    let years_num_expr: Vec<syn::Expr> = years.iter().map(|e| e.replace("y", "")).map(|e| syn::parse_str::<syn::Expr>(&e).unwrap()).collect();

    let auto_import_code = quote! {
        //! Auto-generated file by build script, do not edit!
        #(
            #[path = #years_mod]
            pub mod #years_expr;
        )*

        pub fn select_function(year: u32, day: u32, part: u32) -> fn(String) -> String {
            match year {
                #(#years_num_expr => #years_expr::select_function(day, part),)*
                _ => panic!("Invalid year!"),
            }
        }
    };

    write_and_format(auto_import_code.to_string(), &auto_import_file)
}

fn write_and_format(text: String, path: &PathBuf) {
    let mut file: File = File::create(&path).unwrap();
    file.write_all(text.as_bytes()).unwrap();
    
    // run rustfmt TODO: make sure rustfmt is installed
    // let exit_status = Command::new("rustfmt")
    //     .arg(&path)
    //     .spawn().expect("rustfmt failed to run")
    //     .wait().unwrap();
    // if !exit_status.success() {
    //     println!("cargo:warning=failed to format {:?}", file);
    // }
}
