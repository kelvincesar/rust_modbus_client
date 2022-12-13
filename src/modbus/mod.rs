pub mod base;
pub mod tcp;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tcp_read_holding_register(){
        use base::MbClientBase;
    
        let mut client = tcp::Client::new(String::from("localhost"), 502, 1);
        let result = client.read_holding_registers(0, 10);
        println!("tcp_read_holding_register: {:?}", result);
        client.disconnect();
        
    }
}
