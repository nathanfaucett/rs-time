extern crate time;


use std::thread;
use std::time::Duration;

use time::Time;


#[test]
fn test() {
    let mut time = Time::new();

    time.update();

    thread::sleep(Duration::from_millis(16));
    assert_eq!(time.fps(), 60f64);

    time.update();

    assert_eq!(time.frame(), 2);
    assert_eq!(time.fps(), 60f64);
}
