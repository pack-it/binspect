# Binspect

A simple inspector tool to inspect several types of binaries.

Binspect uses [LIEF](https://github.com/lief-project/LIEF) to load the different binaries.

## Supported binaries and operations

Binspect supports inspecting binaries and simple change operations for the following binaries:
- `MachO`: changing RPath and libraries.
- `ELF`: changing RPath, RunPath and libraries.
- `PE`: changing imports.


## License
The Binspect repository is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for the full license.


## Usage

Binspect uses a simple command structure, consisting of: `binspect <BINARY-PATH> [SUBCOMMANDS] [OPTIONS]`

When using no subcommand, the binary is inspected and fields of the binary, such as libraries, RPath and RunPath are shown.
The flag `--show-flags` can be used to show flags in the binary.

### Available subcommands

#### `binspect <BINARY-PATH> rpath add <RPATH> [--force]`
Adds the given path to the binary. When `--force` is given, the rpath is added, even if it already exists.
Note that this is only available for `ELF` and `MachO` binaries.

#### `binspect <BINARY-PATH> rpath change <OLD-RPATH> <NEW-RPATH>`
Changes the rpath with the given old value to the given new value.
Note that this is only available for `ELF` and `MachO` binaries.

#### `binspect <BINARY-PATH> rpath remove <RPATH>`
Removes the given path from the binary.
Note that this is only available for `ELF` and `MachO` binaries.

#### `binspect <BINARY-PATH> runpath add <RUNPATH> [--force]`
Adds the given path to the binary. When `--force` is given, the runpath is added, even if it already exists.
Note that this is only available for `ELF` binaries.

#### `binspect <BINARY-PATH> runpath change <OLD-RUNPATH> <NEW-RUNPATH>`
Changes the runpath with the given old value to the given new value.
Note that this is only available for `ELF` binaries.

#### `binspect <BINARY-PATH> runpath remove <RUNPATH>`
Removes the given path from the binary.
Note that this is only available for `ELF` binaries.

#### `binspect <BINARY-PATH> library add <LIBRARY> [--force]`
Adds the given library to the binary. When `--force` is given, the library is added, even if it already exists.
Note that libraries are called imports in `PE` binaries.

#### `binspect <BINARY-PATH> library change <OLD-LIBRARY> <NEW-LIBRARY>`
Changes the library with the given old value to the given new value.
Note that libraries are called imports in `PE` binaries.

#### `binspect <BINARY-PATH> library remove <LIBRARY>`
Removes the given library from the binary.
Note that libraries are called imports in `PE` binaries.

#### `binspect <BINARY-PATH> sign`
Sign the given binary.
Note that this is currently only supported for `MachO` binaries.
