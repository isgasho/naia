use crate::{instant::Instant, sequence_buffer::SequenceNumber};

#[derive(Debug, Clone)]
pub struct RttData {
    pub sequence: SequenceNumber,
    pub sending_time: Instant,
}

impl RttData {
    pub fn new(sequence: SequenceNumber) -> Self {
        RttData {
            sequence,
            sending_time: Instant::now(),
        }
    }
}

impl Default for RttData {
    fn default() -> Self {
        RttData {
            sequence: 0,
            sending_time: Instant::now(),
        }
    }
}
