use std::{
    fmt,
    time::Duration,
};

#[derive(Debug)]
pub struct Measure {
    name: &'static str,
}

impl Measure {
    pub fn start(name: &'static str) -> Self {
        Self {
            name,
        }
    }

    pub fn stop(&mut self) {
    }

    pub fn as_ns(&self) -> u64 {
        0
    }

    pub fn as_us(&self) -> u64 {
        0
    }

    pub fn as_ms(&self) -> u64 {
        0
    }

    pub fn as_s(&self) -> f32 {
        0.0
    }

    pub fn as_duration(&self) -> Duration {
        Duration::from_nanos(0)
    }

    pub fn end_as_ns(self) -> u64 {
        0
    }

    pub fn end_as_us(self) -> u64 {
        0
    }

    pub fn end_as_ms(self) -> u64 {
        0
    }

    pub fn end_as_s(self) -> f32 {
        0.0
    }

    pub fn end_as_duration(self) -> Duration {
        Duration::from_nanos(0)
    }
}

impl fmt::Display for Measure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} took {}ns", self.name, 0)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, std::thread::sleep};

    #[test]
    fn test_measure() {
        let test_duration = Duration::from_millis(100);
        let mut measure = Measure::start("test");
        sleep(test_duration);
        measure.stop();
        assert!(measure.as_duration() >= test_duration);
    }

    #[test]
    fn test_measure_as() {
        let test_duration = Duration::from_millis(100);
        let measure = Measure {
            name: "test",
            start: Instant::now(),
            duration: test_duration.as_nanos() as u64,
        };

        assert!(f32::abs(measure.as_s() - 0.1f32) <= f32::EPSILON);
        assert_eq!(measure.as_ms(), 100);
        assert_eq!(measure.as_us(), 100_000);
        assert_eq!(measure.as_ns(), 100_000_000);
        assert_eq!(measure.as_duration(), test_duration);
    }

    #[test]
    fn test_measure_display() {
        let measure = Measure {
            name: "test_ns",
            start: Instant::now(),
            duration: 1,
        };
        assert_eq!(format!("{measure}"), "test_ns took 1ns");

        let measure = Measure {
            name: "test_us",
            start: Instant::now(),
            duration: 1000,
        };
        assert_eq!(format!("{measure}"), "test_us took 1us");

        let measure = Measure {
            name: "test_ms",
            start: Instant::now(),
            duration: 1000 * 1000,
        };
        assert_eq!(format!("{measure}"), "test_ms took 1ms");

        let measure = Measure {
            name: "test_s",
            start: Instant::now(),
            duration: 1000 * 1000 * 1000,
        };
        assert_eq!(format!("{measure}"), "test_s took 1.0s");

        let measure = Measure::start("test_not_stopped");
        assert_eq!(format!("{measure}"), "test_not_stopped running");
    }
}
