# rspinner

> rust spinner

### Install

See [Cargo page](https://crates.io/crates/rspinner)

### Usage

```rust
use rspinner::Spinner;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut spinner = Spinner::new(Some("new spinner..."));

    spinner.start(Some("Start -- 1"));

    sleep(Duration::from_secs(2));

    spinner.success(Some("Success!"));

    spinner.start(Some("Start -- 2"));

    sleep(Duration::from_secs(2));

    spinner.error(Some("Has Error!"));
}
```

### Example

```bash
cargo run --example spinner
```
