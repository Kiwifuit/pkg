mod internal;

use clap::{AppSettings, CommandFactory, ErrorKind, Parser};
use internal::Package;
use std::fmt::Debug;

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
