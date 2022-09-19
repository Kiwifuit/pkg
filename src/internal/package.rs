use crate::internal::utils::*;

use glob::glob;
use std::error::Error;
use std::fmt::Debug;
use std::path::PathBuf;
use std::str::FromStr;

use zip::write::FileOptions;

pub struct Package {
    path: PathBuf,
    zip_options: Option<FileOptions>,
}

impl Package {
    pub fn new(path: &str) -> Option<Self> {
        let path = PathBuf::from(path);

        if !(path.exists() && path.is_dir()) {
            return None;
        }

        Some(Self {
            path: path,
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
        let mut writer = new_zip_file(&self.path)?;

        if let None = self.zip_options {}

        for file in self.search()? {
            let delta = get_delta(&file)?;

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
                let (bytes_written, content_length) =
                    add_to_zip(&file, &mut writer, &delta, &self.zip_options.unwrap())?;

                print!(
                    "OK ({}/{} | {:.2}%)\n",
                    bytes_written,
                    content_length,
                    safe_divide(bytes_written, content_length) * 100
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
