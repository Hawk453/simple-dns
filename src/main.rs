//use std::{fs::File, io::Read};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

use std::net::UdpSocket;

use simple_dns::{BytePacketBuffer, DnsPacket, QueryType, DnsQuestion};

fn main() -> Result<()>{
    let qname = "www.yahoo.com";
    let qtype = QueryType::A;
    let server = ("8.8.8.8", 53); // public google server
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?; // Arbitrary Port
    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet.questions.push(
        DnsQuestion::new(qname.to_string(), qtype)
    );

    let mut buffer = BytePacketBuffer::new();
    packet.write(&mut buffer)?;
    socket.send_to(&buffer.buf[0..buffer.pos], server)?;
    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf)?;
    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;

    println!("{:#?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:#?}", q);
    }

    for rec in res_packet.answers {
        println!("{:#?}", rec);
    }

    for rec in res_packet.authorities {
        println!("{:#?}", rec);
    }

    for rec in res_packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
