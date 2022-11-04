// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use time::{Duration, OffsetDateTime};

#[derive(PartialEq, Debug)]
pub struct Timer {
    real_initial_datetime: OffsetDateTime,
    elapsed: Duration,

    virt_initial_datetime: OffsetDateTime,
    speed: f32,
}

impl Timer {
    pub fn new(virt_initial_datetime: OffsetDateTime, speed: f32) -> Self {
        assert!(speed > 0.0, "speed must be greater then 0");

        let real_initial_datetime = OffsetDateTime::now_utc();
        let elapsed = Duration::seconds(0);
        Timer {
            real_initial_datetime,
            elapsed,
            virt_initial_datetime,
            speed,
        }
    }

    pub fn virt_current_datetime(&mut self) -> OffsetDateTime {
        self.update_clock();
        self.virt_initial_datetime + self.elapsed
    }

    fn update_clock(&mut self) {
        let now = OffsetDateTime::now_utc();
        self.elapsed = (now - self.real_initial_datetime) * self.speed;
    }
}
