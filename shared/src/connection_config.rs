use std::{default::Default, time::Duration};

/// Contains Config properties which will be used by a Server or Client
#[derive(Clone, Debug)]
pub struct ConnectionConfig {
    /// The duration to wait for communication from a remote host before
    /// initiating a disconnect
    pub disconnection_timeout_duration: Duration,
    /// The duration to wait before sending a heartbeat message to a remote
    /// host, if the host has not already sent another message within that time.
    pub heartbeat_interval: Duration,
    /// Value that specifies the factor used to smooth out network jitter. It
    /// defaults to 10% of the round-trip time. It is expressed as a ratio, with
    /// 0 equal to 0% and 1 equal to 100%.
    pub rtt_smoothing_factor: f32,
    /// Value which specifies the maximum round trip time before we consider it
    /// a problem. This is expressed in milliseconds.
    pub rtt_max_value: u16,
}

impl ConnectionConfig {
    /// Creates a new ConnectionConfig, used to initialize a Connection
    pub fn new(
        disconnection_timeout_duration: Duration,
        heartbeat_interval: Duration,
        rtt_smoothing_factor: f32,
        rtt_max_value: u16,
    ) -> Self {
        ConnectionConfig {
            disconnection_timeout_duration,
            heartbeat_interval,
            rtt_smoothing_factor,
            rtt_max_value,
        }
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            disconnection_timeout_duration: Duration::from_secs(10),
            heartbeat_interval: Duration::from_secs(4),
            rtt_smoothing_factor: 0.10,
            rtt_max_value: 250,
        }
    }
}