use std::env::current_dir;
use std::error::Error;
use std::fs::{read, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use zip::{write::FileOptions, CompressionMethod, ZipWriter};

pub fn safe_divide(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    a / b
}

pub fn str_to_compression_method(meth: String) -> CompressionMethod {
    match meth.to_lowercase().as_str() {
        "aes" => CompressionMethod::Aes,
        "bz2" => CompressionMethod::Bzip2,
        "deflate" => CompressionMethod::Deflated,
        "zstd" => CompressionMethod::Zstd,
        "store" | _ => CompressionMethod::Stored,
    }
}

pub fn get_delta(file: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let current_dir = current_dir()?;

    Ok(PathBuf::from(
        file.to_str()
            .unwrap()
            .strip_prefix(current_dir.to_str().unwrap())
            .unwrap(),
    ))
}

pub fn add_to_zip(
    file: &PathBuf,
    writer: &mut ZipWriter<File>,
    delta: &PathBuf,
    options: &FileOptions,
) -> Result<(usize, usize), Box<dyn Error>> {
    let contents = read(&file)?;

    writer.start_file(delta.to_str().unwrap(), *options)?;

    Ok((writer.write(&contents)?, contents.len()))
}

pub fn new_zip_file(path: Box<Path>) -> Result<ZipWriter<File>, Box<dyn Error>> {
    let file = File::create(path.to_str().unwrap().to_owned() + ".zip")?;
    Ok(ZipWriter::new(file))
}
