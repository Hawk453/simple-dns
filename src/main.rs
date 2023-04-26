type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize
}

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

fn main() {
    println!("Hello, world!");
}
