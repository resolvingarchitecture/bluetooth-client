// use blurz::bluetooth_adapter::BluetoothAdapter;
// use blurz::bluetooth_device::BluetoothDevice;
// use blurz::BluetoothSession;

/// Send Message using Bluetooth Radio
pub fn send_message(msg: String) {
    // let path:
    // let session: BluetoothSession = BluetoothSession::create_session(&path).unwrap();
    // let adapter: BluetoothAdapter = BluetoothAdapter::init(&session).unwrap();
    // let device: BluetoothDevice = adapter.get_first_device().unwrap();
    // println!("{:?}", device);
}

/// Receive Message from Bluetooth Radio
fn receive_message(msg: String) {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
