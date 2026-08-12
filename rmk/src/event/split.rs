//! Split keyboard events

use core::sync::atomic::{AtomicU16, Ordering};

use rmk_macro::event;

use super::{battery::BatteryStatusEvent, publish_event};

const USER_STATE_SLOTS: usize = 32;
const USER_STATE_VALID: u16 = 1 << 8;
static USER_STATE: [AtomicU16; USER_STATE_SLOTS] = [const { AtomicU16::new(0) }; USER_STATE_SLOTS];

/// Peripheral connected state changed event
#[event(channel_size = crate::PERIPHERAL_CONNECTED_EVENT_CHANNEL_SIZE, pubs = crate::PERIPHERAL_CONNECTED_EVENT_PUB_SIZE, subs = crate::PERIPHERAL_CONNECTED_EVENT_SUB_SIZE)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PeripheralConnectedEvent {
    pub id: usize,
    pub connected: bool,
}

/// Connected to central state changed event
#[event(channel_size = crate::CENTRAL_CONNECTED_EVENT_CHANNEL_SIZE, pubs = crate::CENTRAL_CONNECTED_EVENT_PUB_SIZE, subs = crate::CENTRAL_CONNECTED_EVENT_SUB_SIZE)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CentralConnectedEvent {
    pub connected: bool,
}

/// Peripheral battery status changed event
#[event(channel_size = crate::PERIPHERAL_BATTERY_EVENT_CHANNEL_SIZE, pubs = crate::PERIPHERAL_BATTERY_EVENT_PUB_SIZE, subs = crate::PERIPHERAL_BATTERY_EVENT_SUB_SIZE)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PeripheralBatteryEvent {
    pub id: usize,
    pub state: BatteryStatusEvent,
}

/// Request a split peripheral to enter its bootloader.
#[event(channel_size = crate::PERIPHERAL_BOOTLOADER_EVENT_CHANNEL_SIZE, pubs = crate::PERIPHERAL_BOOTLOADER_EVENT_PUB_SIZE, subs = crate::PERIPHERAL_BOOTLOADER_EVENT_SUB_SIZE)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PeripheralBootloaderEvent {
    pub id: usize,
}

/// Application-defined byte state synchronized from split central to peripherals.
#[event(channel_size = crate::USER_STATE_EVENT_CHANNEL_SIZE, pubs = crate::USER_STATE_EVENT_PUB_SIZE, subs = crate::USER_STATE_EVENT_SUB_SIZE)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UserStateEvent {
    pub id: u8,
    pub value: u8,
}

/// Update and publish an application-defined split state byte.
///
/// The latest value is retained so a peripheral connecting after the event can
/// receive a complete initial snapshot.
pub fn publish_user_state(id: u8, value: u8) {
    if let Some(state) = USER_STATE.get(id as usize) {
        state.store(USER_STATE_VALID | value as u16, Ordering::Release);
        publish_event(UserStateEvent { id, value });
    }
}

pub(crate) fn user_state(id: u8) -> Option<u8> {
    let state = USER_STATE.get(id as usize)?.load(Ordering::Acquire);
    (state & USER_STATE_VALID != 0).then_some(state as u8)
}

/// Clear BLE peer information event
#[cfg(feature = "_ble")]
#[event(channel_size = crate::CLEAR_PEER_EVENT_CHANNEL_SIZE, pubs = crate::CLEAR_PEER_EVENT_PUB_SIZE, subs = crate::CLEAR_PEER_EVENT_SUB_SIZE)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClearPeerEvent;
