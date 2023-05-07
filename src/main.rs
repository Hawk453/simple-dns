//use std::{fs::File, io::Read};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

use std::net::UdpSocket;

use simple_dns::{BytePacketBuffer, DnsPacket, DnsQuestion, QueryType, ResultCode};

fn lookup(qname: &str, qtype: QueryType) -> Result<DnsPacket> {
    // Forwarding Queries to Public DNS - Google.
    let server = ("8.8.8.8", 53); // public google server
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?; // Arbitrary Port
    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut buffer = BytePacketBuffer::new();
    packet.write(&mut buffer)?;
    socket.send_to(&buffer.buf[0..buffer.pos], server)?;
    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf)?;
    DnsPacket::from_buffer(&mut res_buffer)
}

/// Handle a single incoming packet
fn handle_query(socket: &UdpSocket) -> Result<()> {
    let mut buffer = BytePacketBuffer::new();

    let (_, src) = socket.recv_from(&mut buffer.buf)?;
    let mut request = DnsPacket::from_buffer(&mut buffer)?;

    let mut packet = DnsPacket::new();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;

    if let Some(question) = request.questions.pop() {
        println!("Recived Query: {:?}", question);
        if let Ok(result) = lookup(&question.name, question.qtype) {
            packet.questions.push(question);
            packet.header.rescode = result.header.rescode;

            for rec in result.answers {
                println!("Answers: {:?}", rec);
            }

            for rec in result.authorities {
                println!("Authority: {:?}", rec);
                packet.authorities.push(rec);
            }

            for rec in result.resources {
                println!("Resource: {:?}", rec);
                packet.resources.push(rec);
            }
        } else {
            packet.header.rescode = ResultCode::SERVFAIL;
        }
    } else {
        packet.header.rescode = ResultCode::FORMERR;
    }

    let mut res_buffer = BytePacketBuffer::new();
    packet.write(&mut res_buffer)?;

    let len = res_buffer.pos();
    let data = res_buffer.range_of_bytes(0, len)?;

    socket.send_to(data, src)?;

    Ok(())
}

fn main() -> Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;

    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
