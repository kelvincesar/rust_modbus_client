mod modbus;
use modbus::tcp;

use crate::modbus::base::MbClientBase;

fn main() {
    println!("Hello, world!");
    let mut client = tcp::Client::new(String::from("localhost"), 502, 1);
    let result = client.read_holding_registers(0, 10);
    println!("Result: {:?}", result);

    println!("Client disconnected: {}", client.disconnect());
}
