//! Mobile domain: iOS simulator/emulator lifecycle, Android SDK/scrcpy/logcat,
//! and the shared mobile toolchain detection.

pub mod android_sdk;
pub mod emulator_stream;
pub mod ios_sim_embed;
pub mod ios_sim_native;
pub mod ios_simulator;
pub mod ios_stream;
pub mod iphone_control;
pub mod ios_crosscompile;
pub mod ios_package;
pub mod ios_run;
pub mod iphone_device;
pub mod iphone_deploy;
pub mod iphone_emulator;
pub mod ioscpy_bridge;
pub mod wda_client;
pub mod logcat_service;
pub mod mobile_toolchain;
pub mod scrcpy;
