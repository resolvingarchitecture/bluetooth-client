
// use blurz::bluetooth_adapter::BluetoothAdapter;
// use blurz::bluetooth_device::BluetoothDevice;
// use blurz::BluetoothSession;

use ra_common::models::{Network, Packet};

use log::{trace,info,warn};

pub struct BluetoothClient {

}

impl BluetoothClient {
    pub fn new() -> BluetoothClient {
        BluetoothClient {

        }
    }
    pub fn init(&mut self) {
        info!("{}","Initializing Bluetooth client...")
    }

    pub fn handle(&mut self, packet: &mut Packet) {
        info!("Handling incoming packet id={}",packet.id);
        // let path = packet.to_addr.as_str();
        // let session = BluetoothSession::create_session(Option::Some(path)).unwrap();
        // let adapter = BluetoothAdapter::init(&session).unwrap();
        // let device = adapter.get_first_device().unwrap();
        // info!("{:?}", device);
        // match device.connect(5000) {
        //     Ok(_) => (),
        //     Err(e) => warn!("{:?}", e),
        // }
    }
}

// Send Message using Bluetooth Radio
// pub fn send_message(msg: String) {
//     let path:
//     let session: BluetoothSession = BluetoothSession::create_session(&path).unwrap();
//     let adapter: BluetoothAdapter = BluetoothAdapter::init(&session).unwrap();
//     let device: BluetoothDevice = adapter.get_first_device().unwrap();
//     println!("{:?}", device);
// }

// Receive Message from Bluetooth Radio
// fn receive_message(msg: String) {
//
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
