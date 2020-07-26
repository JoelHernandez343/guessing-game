# Guessing game
Chapter 2, The Rust Programming Language: This code instroduces you to a few common Rust concepts by showing how to use them in a real program!

## Notes
- Rust doc uses `///` and has Markdown support!
- Error handling is with `match` expression and comparing error results of functions like:
```rust
let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
```
- Rust allows shadow name variables!
- Rust crates are controlled by Cargo.toml. It doesn't seem to be console commands like `npm`  :
```shell
npm i package-name // sad 
```