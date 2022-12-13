use std::io::{Error};
use std::time::{SystemTime, UNIX_EPOCH};
type MbAddress = u16;
type MbQuantity = u16;

fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}


pub enum MbFunctions {
    ReadCoils(MbAddress, MbQuantity),
    ReadDiscreteInputs(MbAddress, MbQuantity),
    ReadHoldingRegisters(MbAddress, MbQuantity),
    ReadInputRegisters(MbAddress, MbQuantity)
}
/// Combination of Modbus, IO and data corruption errors
#[derive(Debug)]
pub enum MbErrors {
    Io(Error),
    InvalidResponse,
    InvalidFunction,
    RequestError
}

#[derive(Debug)]
pub struct MbData {
    result: Vec<u16>,
    ts: u64,
    elapsed : Option<u64>
}
impl MbData {
    pub fn new(result: Vec<u16>, elapsed: Option<u64>) -> Self {
        let ts = timestamp() as u64;
        Self {
            result,
            ts,
            elapsed
        }
    }
}


// Basic client structure
pub trait MbClientBase {
    fn connect(&mut self) -> bool;
    fn disconnect(&mut self) -> bool;
    fn set_unit_id(&mut self, uid: u8);
    fn read_holding_registers(&mut self, address: u16, length: u16) -> Result<MbData, MbErrors>;
}


