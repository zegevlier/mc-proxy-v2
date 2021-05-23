use crate::{packet::Packet, plugin, Direction};
use std::{fs::File, time::Instant};

#[derive(Clone)]
pub struct Velocity {
    prev_x: f64,
    prev_y: f64,
    prev_z: f64,
    last_packet_time: Instant,
}

impl plugin::EventHandler for Velocity {
    fn new() -> Self {
        Self {
            prev_x: 0f64,
            prev_y: 0f64,
            prev_z: 0f64,
            last_packet_time: Instant::now(),
        }
    }

    fn on_move(&mut self, x: f64, y: f64, z: f64) -> Option<Vec<(Packet, Direction)>> {
        let distance =
            ((self.prev_x - x).powi(2) + (self.prev_y - y).powi(2) + (self.prev_z - z).powi(2))
                .sqrt();
        let new_time = Instant::now();
        let speed = distance / (new_time - self.last_packet_time).as_secs_f64();
        log::info!("{:.2}", speed);
        self.last_packet_time = new_time;
        self.prev_x = x;
        self.prev_y = y;
        self.prev_z = z;
        None
    }
}
