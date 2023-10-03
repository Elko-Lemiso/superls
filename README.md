# superls

`superls` is a command-line tool written in Rust that provides a visual representation of a repository's structure, displaying directories and files with distinct icons. It offers functionality to filter outputs based on extensions and allows for in-file search with the `grep` capability.
## Features 
- Visual representation with directory (`📁`) and file (`📄`) icons.
- Ability to ignore specific file extensions or directories.
- Option to display only specific file extensions.
- Ability to grep (search) within files for specific patterns.
## Installation

Assuming you have Rust and Cargo installed:

```bash

$ git clone https://github.com/{yourusername}/superls.git
$ cd superls
$ cargo build --release
$ cargo install --path .
```


## Usage

For a quick look at all options:

```bash

$ superls --help
```


### Basic Directory Listing

Navigate to a directory you'd like to inspect and run:

```bash

$ superls .
```


### Filtering Options 
- **Ignore Specific Extensions** :
Ignore files with `.js` and `.css` extensions.

```bash

$ superls . -e js -e css
``` 
- **Only Display Specific Extensions** :
Display only files with `.rs` and `.toml` extensions.

```bash

$ superls . -o rs -o toml
``` 
- **Ignore Specific Directories** :
Ignore directories named `node_modules` and `bin`.

```bash

$ superls . -d node_modules -d bin
```
### Grep (Search) within Files

Search for the string "TODO" in each file:

```bash

$ superls . -g "TODO"
```


### Combining Options

Display only `.js` files, ignoring the `node_modules` directory, and search for "FIXME":

```bash

$ superls . -o js -d node_modules -g "FIXME"
```

