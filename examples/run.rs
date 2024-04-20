use rspinner::Spinner;

use std::{thread::sleep, time::Duration};

fn main() {
    let mut spinner = Spinner::new(None);

    sleep(Duration::from_secs(2));

    spinner.success(None);

    // spinner.error(Some("Has Error!"));

    println!("Hello, world!")
}
