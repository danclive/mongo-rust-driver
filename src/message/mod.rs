use std::io::{Read, Write};
use error::Error::{ArgumentError, ResponseError};
use std::mem;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use error::Result;

#[derive(Debug)]
pub struct Header {
    //message_length: i32,
    pub request_id: i32,
    pub response_to: i32,
    pub op_code: i32
}

#[derive(Debug)]
pub struct Section {
    pub payload_type: u8,
    pub payload: Vec<u8>
}

impl Section {
    pub fn len(&self) -> usize {
        self.payload.len() + 1
    }
}

bitflags! {
    pub struct OpMsgFlags: u32 {
        const CHECKSUM_PRESENT = 0b0_00000000_00000001;
        const MORE_TO_COME     = 0b0_00000000_00000010;
        const EXHAUST_ALLOWED  = 0b1_00000000_00000000;
    }
}

#[derive(Debug)]
pub struct OpMsg {
    pub header: Header,
    pub flag_bits: OpMsgFlags,
    pub sections: Vec<Section>,
    pub checksum: u32
}

impl OpMsg {
    pub fn len(&self) -> usize {
        // header + flags + len of each section + optional checksum
        let mut len = 16 + 4; // header and flag

        for section in &self.sections {
            len += section.len();
        }

        if (self.flag_bits & OpMsgFlags::CHECKSUM_PRESENT).bits() > 0 {
            len += 4;
        }

        len
    }

    pub fn write<W: Write>(&self, buffer: &mut W) -> Result<()> {
        // write header
        buffer.write_i32::<LittleEndian>(self.len() as i32)?;
        buffer.write_i32::<LittleEndian>(self.header.request_id)?;
        buffer.write_i32::<LittleEndian>(self.header.response_to)?;
        buffer.write_i32::<LittleEndian>(self.header.op_code)?;

        // write flags
        buffer.write_u32::<LittleEndian>(self.flag_bits.bits())?;

        // write section
        for section in &self.sections {
            buffer.write_u8(section.payload_type)?;
            buffer.write(&section.payload)?;
        }

        Ok(())
    }

    pub fn read<R: Read>(buffer: &mut R) -> Result<OpMsg> {
        // read header
        let mut len = buffer.read_i32::<LittleEndian>()?;
        let request_id = buffer.read_i32::<LittleEndian>()?;
        let response_to = buffer.read_i32::<LittleEndian>()?;
        let op_code = buffer.read_i32::<LittleEndian>()?;

        println!("len: {:?}", len);

        len -= 16; // header len

        // read flags
        let flags = buffer.read_u32::<LittleEndian>()?;
        let flag_bits = OpMsgFlags::from_bits_truncate(flags);
        len -= 4; // flags len

        let has_checksum = (flag_bits & OpMsgFlags::CHECKSUM_PRESENT).bits() > 0;
        if has_checksum {
            len -= 4;
        }

        let mut sections = Vec::new();
        
        while len > 0 {
            let payload_type = buffer.read_u8()?;
            //let payload_len = buffer.read_i32::<LittleEndian>()?;
            let mut payload_len_buf = [0u8; 4];
            let size = buffer.read(&mut payload_len_buf)?;

            if size < 4 {
                return Err(ResponseError("Expected to read payload_len".to_owned()))
            }

            let payload_len = {
                i32::from(payload_len_buf[0]) |
                i32::from(payload_len_buf[1]) << 8 |
                i32::from(payload_len_buf[2]) << 16 |
                i32::from(payload_len_buf[3]) << 24
            };

            let mut payload_buf = vec![0u8; (payload_len - 4) as usize];
            let size = buffer.read(&mut payload_buf)?;

            if size < (payload_len - 4) as usize {
                return Err(ResponseError("Expected to read payload".to_owned()))
            }

            let mut payload = payload_len_buf.to_vec();
            payload.append(&mut payload_buf);

            let section = Section {
                payload_type,
                payload
            };

            len -= section.len() as i32;

            sections.push(section);
        }

        let checksum  = if has_checksum {
            buffer.read_u32::<LittleEndian>()?
        } else {
            0
        };

        Ok(OpMsg {
            header: Header {
                request_id,
                response_to,
                op_code
            },
            flag_bits,
            sections,
            checksum
        })
    }
}

// struct OpMsgBuilder {
//     request_id: i32,
// }
