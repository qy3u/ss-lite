use std::io::{prelude::*, Result};
use std::net::{SocketAddr, TcpStream};

use crate::cipher::Cipher;

const BufferSize: usize = 1024;

pub struct SecureSocket {
    pub cipher: Cipher,
    pub listen_addr: SocketAddr,
    pub remote_addr: SocketAddr,
}

impl SecureSocket {
    pub fn decode_read(&self, conn: &mut TcpStream, bs: &mut [u8]) -> Result<usize> {
        let n = conn.read(&mut bs[..])?;
        self.cipher.decode(&mut bs[..]);
        Ok(n)
    }

    pub fn encode_write(&self, conn: &mut TcpStream, bs: &mut [u8]) -> Result<usize> {
        self.cipher.encode(&mut bs[..]);
        conn.write(bs)
    }

    pub fn decode_copy(&self, src: &mut TcpStream, dst: &mut TcpStream) -> Result<()> {
        let mut buf = [0; BufferSize];
        while let Ok(n) = self.decode_read(src, &mut buf[..]) {
            self.encode_write(dst, &mut buf[..n])?;
        }
        Ok(())
    }

    pub fn conn_remote(&self) -> Result<TcpStream> {
        TcpStream::connect(&self.remote_addr)
    }
}
