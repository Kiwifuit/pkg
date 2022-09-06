use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{AppSettings, Parser};
use glob::glob;
use zip::ZipWriter;

struct Package {
    path: Box<Path>,
}

impl Package {
    fn new(path: &str) -> Option<Self> {
        let path = Path::new(path);

        if !(path.exists() && path.is_dir()) {
            return None;
        }

        Some(Self {
            path: Box::from(path),
        })
    }

    fn package(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(&self.path)?;
        let mut writer = ZipWriter::new(file);
        let file_list = self.search()?;

        for file in file_list {
            println!("File: {:?}", file);
        }
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
        #[clap(short = 'n', long = "name")]
        name: Package,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Interface::parse();

    println!("{:#?}", cli);
    match cli {
        Interface::Package { name: pkg } => {
            println!("Packaging package {:?}", pkg.package_name());
            for file in pkg.search()? {
                println!("File {:?}", file);
            }
        }
    }

    Ok(())
}
