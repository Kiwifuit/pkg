PKG: A simple utility to package platform-dependent dependencies

Currently, `pkg` only has the `package` subcommand, so here's the documentation to that:

- `-n <NAME>` will have `pkg` search for `NAME` in the current directory. This argument is required

- `-l <LEVEL>` defines the compression level. This can only be between 0 and 9. Defaults to 0

- `-m <METHOD>` defines the compression method. Defaults to 'store'

Available values for the -m flag:

- aes

- bz2

- deflate

- zstd

- store


Note: I don't think I'll be developing this utility any further, as I am contented with it as it is. Feel free to improve upon this repository with a pull request. I'll look into your changes and pull them if I like it