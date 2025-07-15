/*
 * @Author: LeiJiulong
 * @Date: 2025-07-10 17:10:06
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-07-10 19:03:09
 * @Description:
 */

use bytes::{Bytes, BytesMut};
use mini_redis::{Frame, Result, frame::Error::Incomplete};
use tokio::net::TcpStream;
use std::io::Cursor;

// enum Frame {
//     Simple(String),
//     Error(String),
//     Integer(u64),
//     Bulk(Bytes),
//     Null,
//     Array(Vec<Frame>),
// }

// enum HttpFrame {
//     RequestHead {
//         method: Nethod,
//         uri: Uri,
//         version: Version,
//         headers: HeaderMap,
//     },
//     ResponseHead {
//         status: StatusCode,
//         version: Version,
//         headers: HeaderMap,
//     },
//     BodyChunk {
//         chunk: Bytes,
//     }
// }

pub struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: unsize,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: vec![0; 4096],
            cursor: 0,
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(Frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if self.buffer.len() == self.cursor {
                self.buffer.resize(self.cursor * 2, 0);
            }

            let n = self.stream.read(&mut self.buffer).await?;

            if 0 == n {
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            } else {
                self.cursor += n;
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf  = Cursor::new(self.buffer[..]);
        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position as usize;
                buf.set_position(0);
                let frame  = Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            
            Err(Incomplete) => Ok(None),

            Err(e) => Err(e.into()),

        }

    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        
    }
}
