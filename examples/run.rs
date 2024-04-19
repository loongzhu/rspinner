use rspinner::Spinner;

use std::{thread::sleep, time::Duration};

fn main() {
    let spinner = Spinner::new(None);

    sleep(Duration::from_secs(2));

    spinner.success();
}
