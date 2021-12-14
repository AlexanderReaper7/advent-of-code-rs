use std::{io::prelude::*, path::{Path, PathBuf}, fs::File};

fn main () {
    generate_auto_import();
}

fn generate_auto_import() {
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