use std::{env, thread};
use std::time::{Duration, Instant};

use pbr::{ProgressBar, Units};

#[derive(Debug)]
struct Timer {
    name: String,
    duration: Duration,
    start_time: Instant,
}

impl Timer {
    pub fn new(name: String, duration: Duration) -> Self {
        Timer {
            name,
            duration,
            start_time: Instant::now()
        }
    }

    pub fn elapsed_time(&self) -> Duration {
        Instant::now() - self.start_time
    }

    pub fn progress(&self) -> f32 {
        self.elapsed_time().as_secs() as f32 / self.duration.as_secs() as f32 * 100.
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let duration: &str = &args.get(1).expect("Invalid duration");
    let duration = match duration.chars().last() {
        Some(c) if !('0'..='9').contains(&c) => {
            let duration_u64 = duration[0 .. duration.len() - 1].parse::<u64>().expect("Invalid duration");
            match c {
                'h' => Duration::from_secs(duration_u64) * 60 * 60,
                'm' => Duration::from_secs(duration_u64) * 60,
                's' => Duration::from_secs(duration_u64),
                _ => panic!("Invalid unit")
            }
        }
        _ => Duration::from_secs(duration.parse::<u64>().expect("Invalid duration")),
    };
    let default_name = format!("Timer de {:?}", duration);
    let name = args.get(2).unwrap_or(&default_name).to_string();
    let timer = Timer::new(name, duration);
    let nb_step = (timer.duration.as_millis() / 10) as u64;
    let mut pb = ProgressBar::new(nb_step);
    pb.format("[-> ]");
    pb.message(&(String::new() + &timer.name + " "));
    pb.set_width(Some(75));
    pb.show_speed = false;
    pb.show_counter = false;
    pb.show_tick = false;
    thread::spawn(move || {
        for i in 0 .. nb_step {
            thread::sleep(Duration::from_millis(10));
            pb.inc();
            pb.tick()
        }
        pb.finish();
        println!("\nTimer {} is ended", timer.name);
    }).join();
}
