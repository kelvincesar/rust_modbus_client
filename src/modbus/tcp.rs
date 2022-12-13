// External dependencies
use std::io::{Read, Write, Error, ErrorKind};
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};
use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
//use bytes::buf::Writer;


// Internal dependencies
use super::base::{MbClientBase, MbFunctions, MbErrors, MbData};



// Constants
const MB_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const MB_RW_TIMEOUT: Duration = Duration::from_secs(1);



pub struct Client {
    connection: Option<TcpStream>,
    decoder: ModbusRequest,
    ip: String,
    port: u16,
    uid: u8
}

impl Client {
    pub fn new (ip: String, port: u16, uid: u8) -> Client {
        let decoder = ModbusRequest::new(uid, ModbusProto::TcpUdp);

        Self {
            ip,
            port,
            uid,
            connection: None,
            decoder
        }
    }

    fn read (&mut self, function: &MbFunctions) -> Result<MbData, MbErrors> {
        if self.connection.is_none() {
            if !self.connect() {
                return Err(MbErrors::Io(Error::new(ErrorKind::TimedOut, "Could not connect")));
            }
        }

        let mut request = Vec::new();
        let mut response = Vec::new();
        let mut header = [0u8; 6];

        // Request size
        let size = match *function {
            MbFunctions::ReadHoldingRegisters(address, size) => {
                if let Err(e) = self.decoder.generate_get_holdings(address, size, &mut request) {
                    eprintln!("Error on request generation {:?}", e);
                    return Err(MbErrors::RequestError);
                }
                size
                //(a, s,  s * 2 as usize)
            },
            _ => {
                eprintln!("Invalid function");
                return Err(MbErrors::InvalidFunction);
            }
        };
        
        let mut data = Vec::with_capacity(size.into());
        let mut net_pool_time = None;
        if let Some(conn) = &mut self.connection {
            // Request init time
            let start = Instant::now();
            // Send request
            conn.write(&request).expect("Could not send modbus request");

            // Read header
            conn.read_exact(&mut header).expect("Could not read tcp header");
            response.extend_from_slice(&header);

            // Validate response frame
            match guess_response_frame_len(&header, self.decoder.proto) {
                Ok(len) => {
                    if len > 6 {
                        let mut body = vec![0u8; (len - 6) as usize];
                        conn.read_exact(&mut body).unwrap();
                        response.extend(body);
                        
                        // Convert response to u16
                        if let Err(e) = self.decoder.parse_u16(&response, &mut data) {
                            eprintln!("Error on response parsing {:?}", e);
                        } else {
                            net_pool_time = Some(start.elapsed().as_millis() as u64);
                        }
                    }
                },
                Err(e)=> {
                    eprintln!("Error on response frame validation {:?}", e);
                    return Err(MbErrors::InvalidResponse);
                }
            }
        }
        

        // Return data read
        Ok(MbData::new(
            data,
            net_pool_time
        ))
    }
}

impl MbClientBase for Client {
    fn connect(&mut self) -> bool {
        let mut address = (self.ip.to_owned(), self.port).to_socket_addrs().unwrap();
    
        // open TCP connection
        match TcpStream::connect_timeout(&address.next().unwrap(), MB_CONNECT_TIMEOUT) {
            Ok(stream) => {
                stream.set_read_timeout(Some(MB_RW_TIMEOUT)).unwrap();
                stream.set_write_timeout(Some(MB_RW_TIMEOUT)).unwrap();      
                self.connection = Some(stream);    
                return true;   
            },
            Err(e) => {
                eprintln!("error connecting to tcp server {:?}", e);
                self.connection = None;
                return false;
            }
        };
    }
    fn disconnect(&mut self) -> bool {
        if let Some(connection) = &self.connection {
            if let Err(e) = connection.shutdown(Shutdown::Both){
                eprintln!("error closing tcp connection {:?}", e);
                return false
            };
           
        }
        return true
    }

    /// Sets the unit id of modbus device.
    /// 
    /// Arguments:
    /// 
    /// * `uid`: The unit ID of the device.
    fn set_unit_id(&mut self, uid: u8) {
        self.uid = uid;
    }

    fn read_holding_registers(&mut self, addr: u16, count: u16) -> Result<MbData, MbErrors> {
        match self.read(&MbFunctions::ReadHoldingRegisters(addr, count)) {
            Ok(result) =>  Ok(result),
            Err(error) => Err(error)
        }
    }
}