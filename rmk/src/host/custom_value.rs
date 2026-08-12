//! Application boundary for VIA/Vial custom UI values.

use embassy_sync::channel::Channel;

use crate::RawMutex;

const PAYLOAD_LEN: usize = 29;

/// VIA custom-value command forwarded to application code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustomValueCommand {
    Set,
    Get,
    Save,
}

/// A custom-value request. `payload` starts at byte 3 of the VIA packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CustomValueRequest {
    pub command: CustomValueCommand,
    pub channel_id: u8,
    pub value_id: u8,
    pub payload: [u8; PAYLOAD_LEN],
}

/// Application response to a custom-value request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CustomValueResponse {
    /// The request was handled. For Get, the payload is copied to packet byte 3 onward.
    Handled([u8; PAYLOAD_LEN]),
    /// The application does not handle this channel/value.
    Unhandled,
}

static REQUESTS: Channel<RawMutex, CustomValueRequest, 1> = Channel::new();
static RESPONSES: Channel<RawMutex, CustomValueResponse, 1> = Channel::new();

/// Wait for the next custom-value request from the host.
pub async fn receive() -> CustomValueRequest {
    REQUESTS.receive().await
}

/// Reply to the request returned by [`receive`].
pub async fn reply(response: CustomValueResponse) {
    RESPONSES.send(response).await;
}

pub(super) async fn dispatch(request: CustomValueRequest) -> CustomValueResponse {
    REQUESTS.send(request).await;
    RESPONSES.receive().await
}

pub(super) fn payload_from_packet(packet: &[u8; 32]) -> [u8; PAYLOAD_LEN] {
    let mut payload = [0; PAYLOAD_LEN];
    payload.copy_from_slice(&packet[3..]);
    payload
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_starts_after_command_channel_and_value() {
        let mut packet = [0; 32];
        packet[0] = 0x07;
        packet[1] = 9;
        packet[2] = 4;
        packet[3] = 0x12;
        packet[31] = 0x34;

        let payload = payload_from_packet(&packet);
        assert_eq!(payload[0], 0x12);
        assert_eq!(payload[28], 0x34);
    }
}
