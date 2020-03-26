extern crate log;
extern crate simple_logger;

use log::{trace};
use bluetooth_client::BluetoothClient;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting Bluetooth Client Daemon...");
    let mut bt_client = BluetoothClient::new();
    bt_client.init();

    trace!("Bluetooth Client Daemon Stopped.");
}