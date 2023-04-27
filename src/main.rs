use std::net::Ipv4Addr;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize
}
/// Add more records
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType
}
/// DNS Header
#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16, //16byte

    pub recursion_desired: bool, //1 byte
    pub truncated_message: bool,
    pub authoritative_answer: bool,
    pub opcode: u8, // 4 byte
    pub response: bool,

    pub rescode: ResultCode, // 4 byte
    pub checking_disabled: bool,
    pub authed_data: bool,
    pub z: bool,
    pub recursion_available: bool,

    pub questions: u16,
    pub answers: u16,
    pub authoritative_entries: u16,
    pub resource_entries: u16,
}

#[derive(Clone, Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub question: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>
}

/// rescode values
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ResultCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}
/// Record type being queried
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum QueryType {

    UNKNOWN(u16),
    A, // 1

}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
/// to keep track of record types
pub enum DnsRecord{
    UNKNOWN {
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32
    }, // 0
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32
    } // 1

}



// Reading the domain name using BytePacketBuffer
impl BytePacketBuffer {
    /// Buffer for holding packet contents and to keep track of position.
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer { 
            buf: [0; 512],
            pos: 0 
        }
    }

    /// Current position within buffer
    fn pos(&self) -> usize {
        self.pos
    }

    /// Step the buffer position forward by x no. of steps
    fn step(&mut self, steps: usize) -> Result<()> {
        self.pos += steps;
        Ok(())
    }

    /// Change the buffer position
    fn seek(&mut self, pos: usize) -> Result<()> {
        self.pos = pos;
        Ok(())
    }

    /// Read a single byte and move the position 1 step forward
    fn read(&mut self) -> Result<u8> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }

        let single_byte = self.buf[self.pos];
        self.pos += 1;

        Ok(single_byte)
    }

    /// Get a single byte without changing the buffer position
    fn single_byte(&mut self, pos: usize) -> Result<u8> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }

        Ok(self.buf[pos])
    } // get function

    /// Get a range of bytes
    fn range_of_bytes(&mut self, start: usize, len: usize) -> Result<&[u8]> {
        if start + len >= 512{
            return Err("End of buffer".into());
        }

        Ok(&self.buf[start..start + len as usize])
    } // get_range function

    /// Read two bytes, stepping two steps forward
    fn read_u16(&mut self) -> Result<u16> {
        Ok(
            ((self.read()? as u16) << 8) |
            (self.read()? as u16)
        )
    }

    /// Read 4 bytes, stepping 4 steps forward
    fn read_u32(&mut self) -> Result<u32> {
        Ok(
            ((self.read()? as u32) << 24) |
            ((self.read()? as u32) << 16) |
            ((self.read()? as u32) << 8) |
            ((self.read()? as u32) << 0)
        )
    }

    /// Read qname
    fn read_qname(&mut self, outstr: &mut String) -> Result<()> {

        let mut pos = self.pos();
        let mut jumped = false;
        let max_jumps: u8 = 5;
        let mut jumps_performed: u8 = 0;

        let mut delim = "";
        loop {
            if jumps_performed > max_jumps {
                return Err(format!("Limit of {} jumps exeeded", max_jumps).into())
            }

            let len = self.single_byte(pos)?;
            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }
                let byte = self.single_byte(pos + 1)? as u16;
                let offset = (((len as u16)^0xC0) << 8) | byte;
                pos = offset as usize;

                jumped = true;
                jumps_performed += 1;

                continue;
            }
            else {
                pos += 1;
                if len == 0 {
                    break;
                }
                outstr.push_str(delim);

                outstr.push_str(
                    &String::from_utf8_lossy(
                    self.range_of_bytes
                    (pos, len as usize)?
                    )
                    .to_lowercase());

                delim = ".";
                pos += len as usize;

            }
        }

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }


}

impl ResultCode {
    pub fn from_num(num: u8) -> ResultCode {
        match num {
            1 => ResultCode::FORMERR,
            2 => ResultCode::SERVFAIL,
            3 => ResultCode::NXDOMAIN,
            4 => ResultCode::NOTIMP,
            5 => ResultCode::REFUSED,
            0|_ => ResultCode::NOERROR,
        }
    }
}

impl DnsHeader {
    pub fn new() -> DnsHeader {
        DnsHeader { 
            id: 0, 
            recursion_desired: false, 
            truncated_message: false, 
            authoritative_answer: false, 
            opcode: 0, 
            response: false, 
            rescode: ResultCode::NOERROR, 
            checking_disabled: false, 
            authed_data: false, 
            z: false, 
            recursion_available: false, 
            questions: 0, 
            answers: 0, 
            authoritative_entries: 0, 
            resource_entries: 0 
        }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {

        self.id = buffer.read_u16()?;
        let flags = buffer.read_u16()?;
        let a = (flags >> 8) as u8;
        let b = (flags & 0xFF) as u8;

        self.recursion_desired = (a & (1 << 0)) > 0;
        self.truncated_message = (a & (1 << 1)) > 0;
        self.authoritative_answer = (a & (1 << 2)) > 0;
        self.opcode = (a >> 3) & 0x0F;
        self.response = (a & (1 << 7)) > 0;

        self.rescode = ResultCode::from_num(b & 0x0F);
        self.checking_disabled = (b & (1 << 4)) > 0;
        self.authed_data = (b & (1 << 5)) > 0;
        self.z = (b & (1 << 6)) > 0;
        self.recursion_available = (b & (1 << 7)) > 0;

        self.questions = buffer.read_u16()?;
        self.answers = buffer.read_u16()?;
        self.authoritative_entries = buffer.read_u16()?;
        self.resource_entries = buffer.read_u16()?;

        Ok(())
    }
}

impl QueryType {
    pub fn to_num(&self) -> u16 {
        match *self {
            QueryType::UNKNOWN(x) => x,
            QueryType::A => 1,
        }
    }

    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::A,
            _ => QueryType::UNKNOWN(num),
        }
    }
}

impl DnsQuestion {
    pub fn new(name: String, qtype: QueryType) -> DnsQuestion {
        DnsQuestion {
            name, 
            qtype 
        }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {
        buffer.read_qname(&mut self.name)?;
        self.qtype = QueryType::from_num(buffer.read_u16()?);
        let _ = buffer.read_u16()?;

        Ok(())
    }
}

impl DnsRecord {
    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord> {
        let mut domain = String::new();
        buffer.read_qname(&mut domain)?;

        let qtype_num = buffer.read_u16()?;
        let qtype = QueryType::from_num(qtype_num);
        let _ = buffer.read_u16()?;
        let ttl = buffer.read_u32()?;
        let data_len = buffer.read_u16()?;

        match qtype {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                let addr = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8, 
                    ((raw_addr >> 8) & 0xFF) as u8, 
                    ((raw_addr >> 0) & 0xFF) as u8
                );

                Ok(
                    DnsRecord::A { 
                        domain, addr, ttl 
                    }
                )
            }

            QueryType::UNKNOWN(_) => {
                buffer.step(data_len as usize)?;

                Ok(
                    DnsRecord::UNKNOWN { 
                        domain, qtype: qtype_num, data_len, ttl
                    }
                )
            }
        }
    }
}

impl DnsPacket {
    pub fn new() -> DnsPacket {
        DnsPacket { 
            header: DnsHeader::new(), 
            question: Vec::new(), 
            answers: Vec::new(), 
            authorities: Vec::new(), 
            resources: Vec::new() 
        }
    }

    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket> {
        let mut result = DnsPacket::new();
        result.header.read(buffer)?;

        for _ in 0..result.header.questions {
            let mut question = DnsQuestion::new("".to_string(), QueryType::UNKNOWN(0));
            question.read(buffer)?;

            result
            .question
            .push(question);
        }

        for _ in 0..result.header.answers {
            result
            .answers
            .push(DnsRecord::read(buffer)?);
        }

        for _ in 0..result.header.authoritative_entries {
            result
            .authorities
            .push(DnsRecord::read(buffer)?);
        }

        for _ in 0..result.header.resource_entries {
            result
            .resources
            .push(DnsRecord::read(buffer)?);
        }

        Ok(result)

    }
}

fn main() {
    println!("Hello, world!");
}
