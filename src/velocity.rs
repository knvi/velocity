use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Velocity {
    pub worker_id: i64,
    pub sequence_number: i64,
}

#[derive(Clone, Debug)]
pub struct DecodedVelocity {
    pub id: i64,
    pub timestamp: i64,
    pub worker_id: i64,
    pub sequence_number: i64,
}

const EPOCH_START: i64              = 1640995200000;
static SEQUENCE_COUNTER: AtomicI64  = AtomicI64::new(0);


impl Velocity {
    pub fn new(worker_id: i64) -> Self {
        Self {
            worker_id,
            sequence_number: 0,
        }
    }

    /// Generates a velocity id
    pub fn gen(&mut self) -> i64 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64 - EPOCH_START;
        let sequence_number = SEQUENCE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let velocity_id = ((timestamp << 23)
            | (self.worker_id << 13)
            | (sequence_number << 3))
            as i64;
        velocity_id
    }
    
    /// Decodes a velocity id, onto a DecodedVelocity struct.
    pub fn decode(&mut self, id: i64) -> DecodedVelocity {
        let timestamp = ((id >> 23) & 0x7FFFFF0000000) + EPOCH_START;
        let worker_id = (id >> 13) & 0x7FF;
        let sequence_number = id & 0x7;
        DecodedVelocity {
            id,
            timestamp,
            worker_id,
            sequence_number,
        }
    }
}