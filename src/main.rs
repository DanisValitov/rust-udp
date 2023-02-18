use std::env;
use std::error::Error;
use std::io::{stdin, Read};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use byteorder::{ByteOrder, BigEndian};



#[tokio::main]
async fn main() -> std::io::Result<()> {
    let sock = UdpSocket::bind("192.168.1.183:7777").await?;
    // let ue_sock = UdpSocket::bind("0.0.0.0:8888").await?;

    let remote_addr = "192.168.1.155:5566"; //orange pi
        // let remote_addr = "192.168.1.153:5555"; 

    sock.connect(remote_addr).await?;

    let remote_addr_ue = "127.0.0.1:4444";
    // ue_sock.connect(remote_addr_ue).await?;

    let mut buf = [0; 28];

    let len = sock.send("hello".as_bytes()).await?;
    println!("{:?} bytes sent", len);
    loop {
    
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{}/{}-> {:?}",len, addr, buf);

        let x = BigEndian::read_f32(&buf[0..4]);
        let y = BigEndian::read_f32(&buf[4..8]);
        let z = BigEndian::read_f32(&buf[8..12]);
        let w = BigEndian::read_f32(&buf[12..16]);

        let xt = BigEndian::read_f32(&buf[16..20]);
        let yt = BigEndian::read_f32(&buf[20..24]);
        let zt = BigEndian::read_f32(&buf[24..28]);

        println!("{}, {}, {}, {} | {}, {}, {}", x, y, z, w, xt, yt, zt);

        let mut new_arr: [u8; 32] = [0; 32]; // this works for any array size
        let num: i32 = 1;
        let num_b = num.to_be_bytes();
        
        new_arr[0..4].clone_from_slice(&num_b);

        new_arr[4..8].clone_from_slice(&buf[0..4]);
        new_arr[8..12].clone_from_slice(&buf[4..8]);
        new_arr[12..16].clone_from_slice(&buf[8..12]);
        new_arr[16..20].clone_from_slice(&buf[12..16]);

        new_arr[20..24].clone_from_slice(&buf[16..20]);
        new_arr[24..28].clone_from_slice(&buf[20..24]);
        new_arr[28..32].clone_from_slice(&buf[24..28]);

        // let len = ue_sock.send(&new_arr).await?;

    }
}