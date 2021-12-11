# Development of convdate

*written on 2021/12/11*

This is a document for developer of convdate.


Development environment
-----------------------
- cargo 1.56.0 (Maybe even a higher version)
- rustc 1.56.1 (Maybe even a higher version)
- git
- Visual Studio Code (vscode)
    - [Rust](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker) - extension of vscode
    - [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker) - extension of vscode


Repository Structure
--------------------
I show overview below.
```
.
├── .cargo      - configures for cargo
├── .cspell     - configures for Code Spell Checker, extension of vscode
├── .vscode     - configures for Visual Studio Code
├── local       - each developer's working directory: written in .gitignore
├── src         - the source code directory
│   ├── bin/*   - the source code of the binary crates
│   ├── lin.rs  - the source code of the library crate
│   └── etc.    - modules
├── target      - directory of build result: written in .gitignore
├── .gitignore  - (Read the document of git.)
├── Cargo.lock  - the lock file of package versions
├── Cargo.toml  - the package definition
└── README.md   - readme file for user
```


Crates
------
This package has one library crate and some binary crates. The subject of this package is binary crates, and library crates is just a collection of features commonly used by binary crates (at least as of ver.0.3.0).

However, above sentence does not preclude the library crate from becoming a good library that provides time system conversion capabilities with future improvements.


Tests
-----
All tests can run just with `cargo test`.

The tests don't require any resource files to run anywhere. If we create tests that uses a resource file, we should still consider running it everywhere.

In addition, the tests should be exhaustive to prevent degression during refactoring. 

Currently, unit tests and main functions tests are described in each .rc file with `#[cfg(test)]`. If we lose uniformity unnecessarily, it will be difficult to understand the structure of the project, so the tests we create in the future must be similar.


Document
--------
This package's document is written with [doc comments](https://doc.rust-lang.org/rust-by-example/meta/doc.html). It is build as HTML documents by below command.

```bash
cargo doc --no-deps
# or
cargo d    # alias of above command defined in .cargo/config file
```

`--no-deps` option is to avoid generating documentation for external libraries that convdate depend on. 

This command generates the documents into `./target/doc`. To publish the document, it has to be copied to `./docs` directory. This operation will be done **manually** as part of the work on the new version release. In the future, it can be automated by github action, etc.


Release
-------
*As of Dec. 2021, it is not under consideration for publication on crates.io.*

When releasing the software, the following steps should be taken. (*They can be automated in the future.*)

1. Check out `develop` branch and make sure that the source code is what you want to release. In other words, make sure there are no branches that you forgot to merge into `develop`.

1. Run `cargo check` to make sure that no compile errors occur.

1. Run `cargo test` and make sure that all tests pass.

1. Create new branch `release` and checkout it.

1. Update `version` value in `./Cargo.toml`

1. Run `cargo build --release` on Win and Linux to get executable binary files. (the binary files will be created in `./target/release`)

1. Make zipped file of the executable binary files by OS (Win or Linux). These will be published on GitHub later. In other words, they are not contained in the git repository.
    - Each zip file should contains executable binary files
    - Each zip file should contains no other files

1. Run `cargo d` to build the document. (the document will be created in `./target/doc`)

1. Delete the current document `./docs/` and copy the new document `./target/doc/` to `./docs/`.

1. Commit changes.

1. Merge `release` into `develop` and delete `release` branch (both of local and remote).

1. Merge `develop` into `main` and delete `develop` branch (both of local and remote).

1. Create a release on GitHub
    - Make tag at `main` branch.
    - The tag name should contain the version name; for example, "V0.1.0".
    - It should contain the zip files of executable binary files created in above step.
    - You should write the changes as description.
