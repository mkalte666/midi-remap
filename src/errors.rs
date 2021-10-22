use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PortOpenError {
    port_name : String
}

impl PortOpenError {
    pub fn from_port(name : &str) -> PortOpenError {
        PortOpenError {
            port_name: name.to_owned()
        }
    }
}

impl fmt::Display for PortOpenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not open port {}", self.port_name)
    }
}

impl error::Error for PortOpenError {}

#[derive(Debug, Clone)]
pub struct PortLostError {
    port_name : String
}

impl PortLostError {
    pub fn from_port(name : &str) -> PortLostError {
        PortLostError {
            port_name: name.to_owned()
        }
    }
}

impl fmt::Display for PortLostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lost port {}", self.port_name)
    }
}

impl error::Error for PortLostError {}