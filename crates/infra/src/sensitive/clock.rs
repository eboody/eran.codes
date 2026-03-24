use super::*;

#[derive(Default)]
pub struct SystemClock;

impl SystemClock {
    pub fn new() -> Self {
        Self
    }
}

impl sensitive::Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}
