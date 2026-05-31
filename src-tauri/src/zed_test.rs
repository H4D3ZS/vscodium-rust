use gpui::AppContext;
use std::sync::Arc;

pub fn start() {
    std::thread::spawn(|| {
        let app = gpui::Config::default();
        // Wait, how do we initialize headless app?
        println!("Headless GPUI started");
    });
}
