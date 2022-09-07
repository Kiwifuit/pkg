use crate::internal::utils::*;

use glob::glob;
use std::env::current_dir;
use std::error::Error;
use std::fmt::Debug;
use std::fs::{read, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use zip::{write::FileOptions, CompressionMethod, ZipWriter};

pub struct Package {
    path: Box<Path>,
    zip_options: Option<FileOptions>,
}

impl Package {
    pub fn new(path: &str) -> Option<Self> {
        let path = Path::new(path);

        if !(path.exists() && path.is_dir()) {
            return None;
        }

        Some(Self {
            path: Box::from(path),
            zip_options: None,
        })
    }

    pub fn set_options(&mut self, compression_level: i32, method: String) {
        self.zip_options = Some(
            FileOptions::default()
                .compression_level(Some(compression_level))
                .compression_method(str_to_compression_method(method)),
        );
    }

    pub fn package(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(self.path.to_str().unwrap().to_owned() + ".zip")?;
        let mut writer = ZipWriter::new(file);
        let file_list = self.search()?;

        if let None = self.zip_options {}

        for file in file_list {
            let delta = get_delta(file)?;

            print!(
                "[{}] {:?}",
                if file.is_dir() { "D" } else { "F" },
                file.to_str().unwrap()
            );

            if file.is_dir() {
                writer.add_directory(delta.to_str().unwrap(), self.zip_options.unwrap())?;
                print!("\n");
            } else {
                print!("...");
                let (bytes_written, contentLen) =
                    add_to_zip(&file, &mut writer, &delta, &self.zip_options.unwrap())?;

                print!(
                    "OK ({}/{} | {:.2}%)\n",
                    bytes_written,
                    contentLen,
                    safe_divide(bytes_written, contentLen) * 100
                );
            }
        }

        writer.finish()?;
        Ok(())
    }

    fn search(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let path = self.to_string() + "/**/*";
        let raw = glob(path.as_str())?;

        let res = raw.map(|path| path.unwrap()).collect::<Vec<PathBuf>>();

        Ok(res)
    }

    pub fn package_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }
}

impl FromStr for Package {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::new(&s) {
            Some(res) => Ok(res),
            None => Err(format!("{:?} does not exist or is not a directory", s)),
        }
    }
}

impl ToString for Package {
    fn to_string(&self) -> String {
        let path = self.path.canonicalize().unwrap();

        path.to_str().unwrap().to_owned()
    }
}

impl Debug for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Package {:?} on {:?}",
            self.package_name(),
            self.path
                .canonicalize()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
        )
    }
}
