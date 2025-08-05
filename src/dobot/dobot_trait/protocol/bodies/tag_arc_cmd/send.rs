use crate::dobot::dobot_trait::protocol::{Body, protocol_error::ProtocolError};

/// Represents a point in 3D space with an additional rotation component.
/// This struct corresponds to the Python `tagARCCmd.Point` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}

impl Body for Point {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of four `f32`s, each 4 bytes, totaling 16 bytes.
    fn size(&self) -> usize {
        4 * core::mem::size_of::<f32>()
    }

    /// Packs the `Point` struct into a byte sequence.
    /// It serializes the four `f32` values into the buffer using little-endian byte order.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        buffer[offset..offset + float_size].copy_from_slice(&self.x.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.y.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.z.to_le_bytes());
        offset += float_size;

        buffer[offset..offset + float_size].copy_from_slice(&self.r.to_le_bytes());

        Ok(size)
    }

    /// Unpacks a byte sequence into a `Point` struct.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 4 * core::mem::size_of::<f32>();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let mut offset = 0;
        let float_size = core::mem::size_of::<f32>();

        let mut x_bytes = [0u8; 4];
        x_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let x = f32::from_le_bytes(x_bytes);
        offset += float_size;

        let mut y_bytes = [0u8; 4];
        y_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let y = f32::from_le_bytes(y_bytes);
        offset += float_size;

        let mut z_bytes = [0u8; 4];
        z_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let z = f32::from_le_bytes(z_bytes);
        offset += float_size;

        let mut r_bytes = [0u8; 4];
        r_bytes.copy_from_slice(&buffer[offset..offset + float_size]);
        let r = f32::from_le_bytes(r_bytes);

        Ok(Self { x, y, z, r })
    }
}

/// Represents an ARC command with circular and ending points.
/// This struct corresponds to the Python `tagARCCmd` dataclass.
#[derive(Debug, PartialEq, Clone)]
pub struct TagARCCmd {
    /// Any circular point
    pub circ_point: Point,
    /// Circular ending point
    pub to_point: Point,
}

impl Body for TagARCCmd {
    /// Returns the size of the serialized body in bytes.
    /// This is composed of two `Point` structs, each 16 bytes, totaling 32 bytes.
    fn size(&self) -> usize {
        2 * Point::size(&self.circ_point)
    }

    /// Packs the `TagARCCmd` struct into a byte sequence.
    /// It serializes the `circ_point` and `to_point` structs sequentially.
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, ProtocolError> {
        let size = self.size();
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let point_size = Point::size(&self.circ_point);

        // Serialize the first point
        self.circ_point.serialize(&mut buffer[..point_size])?;

        // Serialize the second point
        self.to_point.serialize(&mut buffer[point_size..])?;

        Ok(size)
    }

    /// Unpacks a byte sequence into a `TagARCCmd` struct.
    /// This corresponds to the Python `unpack` method.
    fn deserialize(buffer: &[u8]) -> Result<Self, ProtocolError> {
        let size = 2 * (4 * core::mem::size_of::<f32>());
        if buffer.len() < size {
            return Err(ProtocolError::BufferTooSmall);
        }

        let point_size = 4 * core::mem::size_of::<f32>();

        // Deserialize the first point
        let circ_point = Point::deserialize(&buffer[..point_size])?;

        // Deserialize the second point
        let to_point = Point::deserialize(&buffer[point_size..])?;

        Ok(Self {
            circ_point,
            to_point,
        })
    }
}
