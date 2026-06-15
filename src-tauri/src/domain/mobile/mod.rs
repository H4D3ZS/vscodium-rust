//! Mobile domain: iOS simulator/emulator lifecycle, Android SDK/scrcpy/logcat,
//! and the shared mobile toolchain detection.

pub mod android_sdk;
pub mod emulator_stream;
pub mod ios_sim_embed;
pub mod ios_sim_native;
pub mod ios_simulator;
pub mod ios_stream;
pub mod iphone_emulator;
pub mod logcat_service;
pub mod mobile_toolchain;
pub mod scrcpy;
