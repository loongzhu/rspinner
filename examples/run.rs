use rspinner::Spinner;

use std::{thread::sleep, time::Duration};

fn main() {
    let mut spinner = Spinner::new(Some("new spinner..."));

    sleep(Duration::from_secs(2));

    spinner.start(Some("Start - 1"));

    sleep(Duration::from_secs(2));

    spinner.success(Some("Success!"));

    // spinner.error(Some("Has Error!"));

    spinner.start(Some("Start - 2"));

    sleep(Duration::from_secs(2));

    spinner.error(Some("Has Error!"));

    sleep(Duration::from_secs(1));

    spinner.warning(None);

    sleep(Duration::from_secs(1));

    spinner.info(None);
}
