use std::{fs::File, io::Read};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

use simple_dns::{BytePacketBuffer, DnsPacket};

fn main() -> Result<()>{
    let mut f = File::open("response_packet_new.txt").expect("Unable to open file");
    let mut buffer = BytePacketBuffer::new();

    f.read(&mut buffer.buf)?;

    let packet =  DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
