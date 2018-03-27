use std::net::TcpStream;
use std::io::Result;


#[derive(Clone)]
pub enum StreamConnector {
    Tcp
}

impl Default for StreamConnector {
    fn default() -> Self {
        StreamConnector::Tcp
    }
}

impl StreamConnector {
    pub fn connect(&self, hostname: &str, port: u16) -> Result<TcpStream> {
        match *self {
            StreamConnector::Tcp => {
                TcpStream::connect((hostname, port))
            }
        }
    }
}
