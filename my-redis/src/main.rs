use std::io::Cursor;

use bytes::{Bytes, BytesMut, Buf};
use tokio::{io::AsyncReadExt, net::TcpStream};
use mini_redis::{Frame, Result};


// enum Frame {
//     Simple(String),
//     Error(String),
//     Integer(u64),
//     Bulk(Bytes),
//     Null,
//     Array(Vec<Frame>),
// }

struct Connection {
    stream: TcpStream,
    buffer: BytesMut
}

impl Connection {
    
   pub fn new(stream: TcpStream) -> Self {
    let mut c = Cursor::new(vec![1,2,3]);
    
    Self { stream, buffer: BytesMut::with_capacity(4096) }
   }

   pub async fn read_frame(&mut self) -> Result<Option<Frame>>{
    loop {
        if let Some(frame) = self.parse_frame()? {
            return Ok(Some(frame));
        }

        if 0 == self.stream.read_buf(&mut self.buffer).await? {
            return Ok(None);
        } else {
            return Err("Connection closed".into());
        }
    }
   }
   
   pub fn parse_frame(&self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);
        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position();
                buf.set_position(0);
                let frame = Frame::parse(&mut buf)?;
                Ok(Some(frame))
            },
            Err(mini_redis::frame::Error::Incomplete) => Ok(None),
        // An error was encountered
        Err(e) => Err(e.into()),
        }
    }
}

fn main() {}