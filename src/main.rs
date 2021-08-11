pub mod gdp;

use crate::gdp::{GdpFile};
use anyhow::{Result, bail};
use std::ffi::OsString;
use std::path::{PathBuf};
use std::env;
use std::fs::{create_dir_all, write};
use std::cmp;

fn get_path_without_filename(str: &String) -> OsString {
    let mut buff = PathBuf::from(str);
    buff.pop();

    buff.into_os_string()
}

// https://github.com/banyan/rust-pretty-bytes/blob/master/src/converter.rs
pub fn pretty_bytes(num: f64) -> String {
    let negative = if num.is_sign_positive() { "" } else { "-" };
    let num = num.abs();
    let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    if num < 1_f64 {
        return format!("{}{} {}", negative, num, "B");
    }
    let delimiter = 1000_f64;
    let exponent = cmp::min((num.ln() / delimiter.ln()).floor() as i32, (units.len() - 1) as i32);
    let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent)).parse::<f64>().unwrap() * 1_f64;
    let unit = units[exponent as usize];
    format!("{}{} {}", negative, pretty_bytes, unit)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: gdp file.gdp");
    }

    let mut gdp_file = GdpFile::open(&args[1])?;

    println!("File: {}\nPatch Info: {}\n\nFiles (total: {}):", &args[1], gdp_file.header.patch_info, gdp_file.header.file_count);
    for ii in 0..gdp_file.header.file_count {
        let entry = &gdp_file.entries[ii as usize];
        println!("{}\n\tFile Size: {}\n", entry.file_path, pretty_bytes(entry.file_size as f64));

        let path = &format!("{}{}", env::current_dir().unwrap().display(), entry.file_path);

        create_dir_all(get_path_without_filename(path))?;
        write(path, gdp_file.extract(ii)?)?;
    }

    Ok(())
}
