# convdate

**convdate** is tools to convert datetime UTC <=> TAI <=> TT.

*It is not related to the linux command of the same name.*


You can:
- convert from some time systems to some time systems
    ```bash
    # UTC -> TAI
    $ ./utc2tai 2017-01-01T00:00:00.000
    # UTC -> TT (TT = TAI + 32.184s)
    $ ./utc2tt 2017-01-01T00:00:00.000
    # TAI -> UTC
    $ ./tai2utc 2017-01-01T00:00:00.000
    # TT -> UTC (TT = TAI + 32.184s)
    $ ./tt2utc 2017-01-01T00:00:00.000
    ```
- convert time systems considering leap seconds
    ```bash
    $ ./tai2utc 2017-01-01T00:00:36.000 2017-01-01T00:00:37.000
    2016-12-31T23:59:60.000
    2017-01-01T00:00:00.000
    ```
- specify datetime format
    ```bash
    $ ./utc2tai --dt-fmt=%Y%m%d%H%M%S 20170101000000
    20170101000037
    ```
- know more features
    ```
    $ ./utc2tai --help
    ```


Install and use (executable file)
---------------------------------

1. Download from https://github.com/unaguna/convdate/releases
1. Unzip downloaded file
1. (except for win) Use `chmod +x` for unzipped files
1. Execute; for example:
    ```bash
    ./utc2tai 2017-01-01T11:22:33 2017-01-02T11:22:33
    ./utc2tt 2017-01-01T11:22:33 2017-01-02T11:22:33
    ./tai2utc 2017-01-01T11:22:33 2017-01-02T11:22:33
    ./tt2utc 2017-01-01T11:22:33 2017-01-02T11:22:33
    ```
1. For more usage, look for help by ``./utc2tai --help``


Install and use (from source code)
----------------------------------
*For Developers*

1. Make sure that `git` and `cargo` are installed.
    ```bash
    git --version
    cargo --version
    ```
    `cargo` is a tool to develop [Rust](https://www.rust-lang.org) programs,
1. Clone this repository
    ```bash
    git clone <url>
    cd convdate
    ```
1. Build executables
    ```bash
    cargo build --release
    ```
1. Execute; for example:
    ```bash
    cd target/release
    ./utc2tai 2017-01-01T11:22:33 2017-01-02T11:22:33
    ./utc2tt 2017-01-01T11:22:33 2017-01-02T11:22:33
    ./tai2utc 2017-01-01T11:22:33 2017-01-02T11:22:33
    ./tt2utc 2017-01-01T11:22:33 2017-01-02T11:22:33
    ```
1. For more usage, look for help by ``./utc2tai --help``
