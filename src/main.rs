use std::env::current_dir;
use std::error::Error;
use std::fmt::Debug;
use std::fs::{read, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::{AppSettings, CommandFactory, ErrorKind, Parser};
use glob::glob;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

struct Package {
    path: Box<Path>,
    zip_options: Option<FileOptions>,
}

impl Package {
    fn new(path: &str) -> Option<Self> {
        let path = Path::new(path);

        if !(path.exists() && path.is_dir()) {
            return None;
        }

        Some(Self {
            path: Box::from(path),
            zip_options: None,
        })
    }

    fn set_options(&mut self, compression_level: i32, method: String) {
        let meth = match method.to_lowercase().as_str() {
            "aes" => CompressionMethod::Aes,
            "bz2" => CompressionMethod::Bzip2,
            "deflate" => CompressionMethod::Deflated,
            "zstd" => CompressionMethod::Zstd,
            "store" | _ => CompressionMethod::Stored,
        };

        self.zip_options = Some(
            FileOptions::default()
                .compression_level(Some(compression_level))
                .compression_method(meth),
        );
    }

    fn package(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(self.path.to_str().unwrap().to_owned() + ".zip")?;
        let mut writer = ZipWriter::new(file);
        let file_list = self.search()?;
        let current_dir = current_dir()?;

        if let None = self.zip_options {}

        for file in file_list {
            let delta = PathBuf::from(
                file.to_str()
                    .unwrap()
                    .strip_prefix(current_dir.to_str().unwrap())
                    .unwrap(),
            );

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
                let contents = read(&file)?;

                writer.start_file(delta.to_str().unwrap(), self.zip_options.unwrap())?;

                let bytes_written = writer.write(&contents)?;
                print!(
                    "OK ({}/{} | {:.2}%)\n",
                    bytes_written,
                    contents.len(),
                    safe_divide(bytes_written, contents.len()) * 100
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

    fn package_name(&self) -> &str {
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

#[derive(Parser, Debug)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
enum Interface {
    Package {
        /// Name of the package
        #[clap(short = 'n', long = "name")]
        name: Package,

        #[clap(short = 'l', long = "level", default_value_t = 0)]
        /// Compression level. This can only be from 0 through 9
        compression_level: i32,

        #[clap(short = 'm', long = "method", default_value = "store")]
        /// Method of compression. Can be "aes", "bz2", "deflate", "zstd", or "store"
        method: String,
    },
}

fn safe_divide(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    a / b
}

fn verify_compression_val(val: &i32) -> bool {
    val >= &0 && val <= &9
}

fn verify_compression_meth(meth: &String) -> bool {
    match meth.to_lowercase().as_str() {
        "aes" => true,
        "bz2" => true,
        "deflate" => true,
        "zstd" => true,
        "store" => true,
        _ => false,
    }
}

fn main() {
    let cli = Interface::parse();
    let mut terminal = Interface::command();

    match cli {
        Interface::Package {
            name: mut pkg,
            compression_level: level,
            method: meth,
        } => {
            if !verify_compression_val(&level) {
                terminal
                    .error(
                        ErrorKind::InvalidValue,
                        format!(
                            "Compression value can be only be between 0 and 9, not {}",
                            level
                        ),
                    )
                    .exit();
            }

            if !verify_compression_meth(&meth) {
                terminal
                    .error(
                        ErrorKind::InvalidValue,
                        format!(
                            "Compression method can only be \"aes\", \"bz2\", \"deflate\", \"zstd\", or \"store\", not {:?}",
                            meth
                        ),
                    )
                    .exit();
            }

            pkg.set_options(level, meth);

            println!(
                "Packaging package {:?} with compress level {}",
                pkg.package_name(),
                level
            );

            match pkg.package() {
                Ok(_) => println!("Package success!"),
                Err(err) => eprintln!("Error during packaging: {}", err),
            };
        }
    }
}
