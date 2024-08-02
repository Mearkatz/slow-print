use std::{
	io::{stdout, Write},
	thread::sleep,
	time::Duration,
};

use rand::{thread_rng, Rng};


/// Prints some text, sleeping a random amount of time between each character.
fn slow_print(s: &str, min_sleep_time: Duration, max_sleep_time: Duration) -> std::io::Result<()> {
	let mut stdout = stdout();
	let mut rng = thread_rng();
	s.chars().try_for_each(|c| {
		print!("{c}");
		sleep(rng.gen_range(min_sleep_time..max_sleep_time));
		stdout.flush()
	})
}

fn main() -> std::io::Result<()> {
	slow_print(include_str!("text.txt"), Duration::from_millis(30), Duration::from_millis(100))
}
