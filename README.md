# Cheerz-dl

Download pictures from a [Cheerz Photobooth](https://live.cheerz.com/location) link.

All you need to have is the private URL of the photo gallery.

The script is written in Rust. Yes it's overkill! But it was a fun challenge.

# Building

```shell script
cargo build
```

# Running

```shell script
cargo run -- --help
```

```text
Cheerz downloader 0.1.0
Farzad FARID <farzy@farzy.org>:Gilles RASIGADE
Download all pictures from a Cheerz event

USAGE:
    cheerz-dl [OPTIONS] <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --directory <directory>    Directory to download pictures to [default: /tmp/cheerz]

ARGS:
    <url>    URL provided by Cheerz for the event
```

# Debugging

Prefix the binary with `RUST_LOG=cheerz_dl=debug` in order to
display debug messages.

Using `RUST_LOG=debug` is more verbose and displays libraries' debug messages too.

# Licence

Copyright 2020 Gilles Rasigade, Farzad FARID

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
