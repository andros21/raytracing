#![doc = include_str!("../README.md")]

mod cli;
mod color;
mod error;
mod hdrimage;
mod misc;
mod normal;
mod point;
mod transformation;
mod vector;

use std::env;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

use crate::error::ConvertErr;
use crate::hdrimage::{HdrImage, Luminosity};

/// Crate main function.
///
/// * parse subcommands and subcommands arguments
/// * call subcommands relative function
/// * check results
/// * print to stderr and exit 1 if error was detected
fn main() {
    let cli_m = cli::build_cli().get_matches_from(env::args_os());
    match cli_m.subcommand_name() {
        Some("convert") => match convert(cli_m.subcommand_matches("convert").unwrap()) {
            Ok(()) => exit(0),
            Err(e) => {
                eprintln!("[error] {:#}", e);
                exit(1)
            }
        },
        _ => exit(1),
    }
}

/// Convert High Dynamic Range (LDR) image to Low Dynamic Range (HDR).
///
/// Called when `rustracer-convert` subcommand is used.
fn convert(sub_m: &clap::ArgMatches) -> Result<(), ConvertErr> {
    let hdr_file = Path::new(sub_m.value_of("HDR").unwrap());
    let ldr_file = Path::new(sub_m.value_of("LDR").unwrap());
    let factor = f32::from_str(sub_m.value_of("factor").unwrap())
        .map_err(|e| ConvertErr::FloatParseFailure(e, String::from("factor")))?;
    let gamma = f32::from_str(sub_m.value_of("gamma").unwrap())
        .map_err(|e| ConvertErr::FloatParseFailure(e, String::from("gamma")))?;
    let mut hdr_img = HdrImage::read_pfm_file(hdr_file).map_err(ConvertErr::IoError)?;
    if sub_m.is_present("verbose") {
        println!("[info] {:?} has been read from disk", hdr_file);
    }
    hdr_img.normalize_image(factor, Luminosity::AverageLuminosity);
    hdr_img.clamp_image();
    hdr_img
        .write_ldr_file(ldr_file, gamma)
        .map_err(ConvertErr::IoError)?;
    if sub_m.is_present("verbose") {
        println!("[info] {:?} has been written to disk", ldr_file);
    }
    Ok(())
}
