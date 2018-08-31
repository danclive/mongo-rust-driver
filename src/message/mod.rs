use std::io::{Read, Write};
use error::Error::{ArgumentError, ResponseError};
use std::mem;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use error::Result;
use bson::Document;
use bson::encode::EncodeResult;

#[derive(Debug, Clone)]
pub struct Header {
    //message_length: i32,
    pub request_id: i32,
    pub response_to: i32,
    pub op_code: i32
}

#[derive(Debug, Clone)]
pub struct Section {
    pub payload_type: u8,
    pub payload: Vec<u8>
}

impl Section {
    pub fn from_document(doc: Document) -> EncodeResult<Section> {
        Ok(Section {
            payload_type: 0,
            payload: doc.to_vec()?
        })
    }

    pub fn from_documents(identifier: &str, docs: Vec<Document>) -> EncodeResult<Section> {
        let mut buf = Vec::new();

        // write size position
        buf.write_i32::<LittleEndian>(0)?;
        // write identifier
        write_cstring(&mut buf, identifier)?;
        // write documents
        for doc in &docs {
            doc.to_writer(&mut buf)?;
        }

        Ok(Section {
            payload_type: 1,
            payload: buf
        })
    }

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

#[derive(Debug, Clone)]
pub struct OpMsg {
    pub header: Header,
    pub flag_bits: OpMsgFlags,
    pub sections: Vec<Section>,
    pub checksum: u32
}

impl OpMsg {
    pub fn builder() -> OpMsgBuilder {
        OpMsgBuilder::new()
    }

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

        let _ = buffer.flush()?;

        Ok(())
    }

    pub fn read<R: Read>(buffer: &mut R) -> Result<OpMsg> {
        // read header
        let mut len = buffer.read_i32::<LittleEndian>()?;
        let request_id = buffer.read_i32::<LittleEndian>()?;
        let response_to = buffer.read_i32::<LittleEndian>()?;
        let op_code = buffer.read_i32::<LittleEndian>()?;

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
                return Err(ResponseError("Expected to read payload_len: the len must be longer then 4 bits".to_owned()))
            }

            let payload_len = {
                i32::from(payload_len_buf[0]) |
                i32::from(payload_len_buf[1]) << 8 |
                i32::from(payload_len_buf[2]) << 16 |
                i32::from(payload_len_buf[3]) << 24
            };

            let mut payload = payload_len_buf.to_vec();

            while payload.len() < payload_len as usize {
                let mut buf = [0; 1024 * 4];

                let size = buffer.read(&mut buf)?;

                payload.write(&buf[0..size])?;
            }

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

pub struct OpMsgBuilder {
    op_msg: OpMsg
}

impl OpMsgBuilder {
    pub fn new() -> OpMsgBuilder {
        OpMsgBuilder {
            op_msg: OpMsg {
                header: Header {
                    request_id: 0,
                    response_to: 0,
                    op_code: 2013
                },
                flag_bits: OpMsgFlags::empty(),
                sections: Vec::new(),
                checksum: 0
            }
        }
    }

    pub fn request_id(&mut self, request_id: i32) -> &mut OpMsgBuilder {
        self.op_msg.header.request_id = request_id;
        self
    }

    pub fn flag_bits(&mut self, flag_bits: OpMsgFlags) -> &mut OpMsgBuilder {
        self.op_msg.flag_bits = flag_bits;
        self
    }

    pub fn sections(&mut self, sections: Vec<Section>) -> &mut OpMsgBuilder {
        self.op_msg.sections = sections;
        self
    }

    pub fn push_section(&mut self, section: Section) -> &mut OpMsgBuilder {
        self.op_msg.sections.push(section);
        self
    }

    pub fn build(&self) -> &OpMsg {
        &self.op_msg
    }
}

fn write_cstring<W>(writer: &mut W, s: &str) -> EncodeResult<()>
    where W: Write + ?Sized
{
    writer.write_all(s.as_bytes())?;
    writer.write_u8(0)?;
    Ok(())
}
