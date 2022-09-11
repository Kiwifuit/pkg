# PKG

A simple utility to package platform-dependent dependencies

# Command-line Usage

Currently, `pkg` only has the `package` subcommand, so here's the documentation to that:

* `-n <NAME>` will have `pkg` search for `NAME` in the current directory. This argument is required
* `-l <LEVEL>` defines the compression level. This can only be between 0 and 9. Defaults to 0
* `-m <METHOD>` defines the compression method.
  * `pkg` accepts the following as valid values for `-m`:
    * `aes`
    * `bz2`
    * `deflate`
    * `zstd`
    * `store`
  * Defaults to `store`

# Compilation

Currently, automated compilation is only possible in Linux, GNU Make is installed on most distributions. Compilation on windows can be possible with either a linux subsystem or by manually building with Cargo

Make `pkg` for windows and linux with `make`.

Additionally, flags/arguments to be passed to Cargo can be modified in the [Makefile](/Makefile)

All compiled executables can be found in `./bin`

# Note

I don't think I'll be developing this utility any further, as I am contented with it as it is.

Feel free to improve upon this repository with a pull request. I'll look into your changes and pull them if I like it