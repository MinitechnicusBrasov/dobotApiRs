pub mod alarm;
pub mod bodies;
mod body;
mod command_id;
mod protocol_error;
pub use body::Body;
pub use command_id::CommunicationProtocolIDs;
pub use protocol_error::ProtocolError;

pub struct Protocol<T> {
    pub command_id: CommunicationProtocolIDs,
    pub is_queued: bool,
    pub is_read: bool,
    pub body: T,
}

impl<T: Body> Protocol<T> {
    pub fn new(
        command_id: CommunicationProtocolIDs,
        is_queued: bool,
        is_read: bool,
        body: T,
    ) -> Self {
        Protocol {
            command_id,
            is_queued,
            is_read,
            body,
        }
    }

    fn calculate_checksum(payload: &[u8]) -> u8 {
        let sum: u8 = payload.iter().copied().sum();
        (!sum).wrapping_add(1)
    }

    pub fn to_packet(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let body_size = self.body.size();

        let body_and_queued_index_size = if self.is_queued {
            if body_size != 0 {
                return Err(ProtocolError::PassedBodyAndQueuedIndex);
            }
            8
        } else {
            body_size
        };

        let content_length = 1 + 1 + body_and_queued_index_size;
        let total_packet_size = 2 + 1 + content_length + 1;

        if buffer.len() < total_packet_size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut index = 0;
        buffer[index] = 0xAA;
        index += 1;
        buffer[index] = 0xAA;
        index += 1;

        let payload_start_index = index;
        buffer[index] = content_length as u8; // Length byte
        index += 1;

        buffer[index] = self.command_id.into(); // Command ID byte
        index += 1;

        let ctrl_byte = (self.is_queued as u8) | ((self.is_read as u8) << 1);
        buffer[index] = ctrl_byte;
        index += 1;

        // Serialize the body directly into the buffer.
        let bytes_written = self.body.serialize(&mut buffer[index..])?;
        index += bytes_written;

        // Calculate the checksum for the payload (length, id, ctrl, body).
        let payload = &buffer[payload_start_index..index];
        let checksum = Self::calculate_checksum(payload);
        buffer[index] = checksum;
        index += 1;

        Ok(index)
    }

    pub fn from_packet(packet: &[u8]) -> Result<Protocol<T>, ProtocolError> {
        if packet.len() < 4 {
            return Err(ProtocolError::BufferTooSmall);
        }

        if packet[0] != 0xAA || packet[1] != 0xAA {
            return Err(ProtocolError::MissingStartBytes);
        }

        let content_length = packet[2] as usize;
        let total_packet_size = 2 + 1 + content_length + 1;

        if packet.len() < total_packet_size {
            return Err(ProtocolError::LengthMismatch);
        }

        let payload = &packet[2..2 + 1 + content_length];
        let received_checksum = packet[2 + 1 + content_length];
        let calculated_checksum = Self::calculate_checksum(payload);

        if received_checksum != calculated_checksum {
            return Err(ProtocolError::ChecksumError);
        }

        let command_id = CommunicationProtocolIDs::try_from(packet[3])?;

        let ctrl_byte = packet[4];

        let is_queued = (ctrl_byte & 0x01) != 0;
        let is_read = (ctrl_byte & 0x02) != 0;

        let body_start_index = 5;
        let body_bytes = &packet[body_start_index..body_start_index + (content_length - 2)];

        let body = T::deserialize(body_bytes)?;

        Ok(Protocol {
            command_id,
            is_queued,
            is_read,
            body,
        })
    }
}
