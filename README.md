# Tion Security Software


Tion Security contains multiple programs
  - [Main server][main_server] written in [Rust][rust]
  - [User interface][userInterface] writting in php
  - Arduino client
  - Raspberry client
 
 
# [Main server][main_server]

### Setup:
Start with installing [Rust][get_rust] and follow the setup guide

Next change rust to the nightly version using the following command:
```sh
cargo install cargo-watch
```

If you are using a development enivorment install cargo-watch to automatic restart when a file changes:
```sh
cargo install cargo-watch
```

### Running the server:
Normal startcommand:
```sh
cargo build
cargo run
```

Development command to detect filechanges:
```sh
cargo watch -x run --clear
```


[rust]: <https://www.rust-lang.org/>
[get_rust]: <https://www.rust-lang.org/>
[main_server]: <https://github.com/onno204/rustc-TioN/tree/master/main%20server/>
[userInterface]: <https://github.com/onno204/rustc-TioN/tree/master/userInterface>
