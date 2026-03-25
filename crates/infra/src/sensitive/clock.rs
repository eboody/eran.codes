use super::*;

#[derive(Default)]
pub struct System;

impl System {
    pub fn new() -> Self {
        Self
    }
}

impl sensitive::Clock for System {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}
