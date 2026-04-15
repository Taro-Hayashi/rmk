//! Exposed channels which can be used to share data across devices & processors

use embassy_sync::channel::Channel;
pub use embassy_sync::{blocking_mutex, channel, pubsub, zerocopy_channel};
#[cfg(feature = "_ble")]
use {crate::ble::profile::BleProfileAction, embassy_sync::signal::Signal, rmk_types::led_indicator::LedIndicator};

use crate::hid::Report;
#[cfg(feature = "storage")]
use crate::{FLASH_CHANNEL_SIZE, storage::FlashOperationMessage};
use crate::{REPORT_CHANNEL_SIZE, RawMutex};

/// Signal for LED indicator, used in BLE keyboards only since BLE receiving is not async
#[cfg(feature = "_ble")]
pub(crate) static LED_SIGNAL: Signal<RawMutex, LedIndicator> = Signal::new();
/// Channel for keyboard report from input processors to hid writer/reader
pub static KEYBOARD_REPORT_CHANNEL: Channel<RawMutex, Report, REPORT_CHANNEL_SIZE> = Channel::new();

/// User key event: (user key id, pressed).
/// Published from the keyboard loop for every User(id) action so downstream
/// code (input processors, user-side tasks) can react to custom keycodes.
pub static USER_KEY_EVENT_CHANNEL: Channel<RawMutex, UserKeyEvent, 4> = Channel::new();

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UserKeyEvent {
    pub id: u8,
    pub pressed: bool,
}

// Sync messages from server to flash
#[cfg(feature = "storage")]
pub(crate) static FLASH_CHANNEL: Channel<RawMutex, FlashOperationMessage, FLASH_CHANNEL_SIZE> = Channel::new();
#[cfg(feature = "_ble")]
pub(crate) static BLE_PROFILE_CHANNEL: Channel<RawMutex, BleProfileAction, 1> = Channel::new();
