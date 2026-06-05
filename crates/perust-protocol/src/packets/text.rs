use crate::error::ProtocolError;
use crate::packet::Packet;
use crate::protocol_info;
use crate::types::TextPacketType;
use perust_utils::{BinaryReader, BinaryWriter};

// ============================================================================
// TextPacket
// ============================================================================

/// Text packet for chat messages and other text communication.
#[derive(Debug, Clone)]
pub struct TextPacket {
    pub text_type: TextPacketType,
    pub source: String,
    pub message: String,
    pub parameters: Vec<String>,
    pub xuid: String,
    pub platform_chat_id: String,
}

impl Packet for TextPacket {
    const PACKET_ID: u8 = protocol_info::TEXT_PACKET;

    fn encode(&self, writer: &mut BinaryWriter) -> Result<(), ProtocolError> {
        self.text_type.encode(writer);

        match self.text_type {
            TextPacketType::Chat => {
                writer.write_string(&self.source);
                writer.write_string(&self.message);
            }
            TextPacketType::Whisper => {
                writer.write_string(&self.source);
                writer.write_string(&self.message);
            }
            TextPacketType::Raw => {
                writer.write_string(&self.message);
            }
            TextPacketType::System => {
                writer.write_string(&self.message);
            }
            TextPacketType::Translation => {
                writer.write_string(&self.message);
                writer.write_u8(self.parameters.len() as u8);
                for param in &self.parameters {
                    writer.write_string(param);
                }
            }
            TextPacketType::Popup => {
                writer.write_string(&self.message);
            }
            TextPacketType::Tip => {
                writer.write_string(&self.message);
            }
        }

        writer.write_string(&self.xuid);
        writer.write_string(&self.platform_chat_id);

        Ok(())
    }

    fn decode(reader: &mut BinaryReader) -> Result<Self, ProtocolError> {
        let text_type = TextPacketType::decode(reader)?;

        let (source, message, parameters) = match text_type {
            TextPacketType::Chat | TextPacketType::Whisper => {
                let source = reader.read_string_owned()?;
                let message = reader.read_string_owned()?;
                (source, message, Vec::new())
            }
            TextPacketType::Raw | TextPacketType::System | TextPacketType::Popup | TextPacketType::Tip => {
                let message = reader.read_string_owned()?;
                (String::new(), message, Vec::new())
            }
            TextPacketType::Translation => {
                let message = reader.read_string_owned()?;
                let param_count = reader.read_u8()? as usize;
                let mut parameters = Vec::with_capacity(param_count.min(256));
                for _ in 0..param_count {
                    parameters.push(reader.read_string_owned()?);
                }
                (String::new(), message, parameters)
            }
        };

        let xuid = reader.read_string_owned()?;
        let platform_chat_id = reader.read_string_owned()?;

        Ok(Self {
            text_type,
            source,
            message,
            parameters,
            xuid,
            platform_chat_id,
        })
    }

    fn packet_name(&self) -> &'static str {
        "TextPacket"
    }
}
