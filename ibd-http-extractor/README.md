# ibd-http-extractor
Extract http(s) links from MySQL's `.ibd` files

## Installation

1. Install Rust
    * If Rust is not installed, please follow [Rust's installation instructions](https://rust-lang.org/tools/install)
   
2. Open a CLI & type 
`cargo install --branch main --git https://github.com/ZimCodes/bonus/ibd-http-extractor`
    
    * This will install **ibd-http-extractor** from *GitHub*

To use, start each command using `ibd` 

## Commands
### Positional

***Paths...***

The file path(s) of the .ibd file(s).

### Options

| Options         | information                |
|-----------------|----------------------------|
| `-h, --help`    | Prints help information    |
| `-V, --version` | Prints version information |
| `-i, --input`   | .ibd directory path        |
| `-o`            | Save location              |

## License
ibd-http-extractor is licensed under the *MIT*.

See [MIT](https://github.com/ZimCodes/bonus/blob/main/LICENSE) for more details
