#[cfg(feature = "std")]
mod test;

use crate::dobot::dobot_trait::{dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::
            calibration_control::CalibrationControl
        ,
    }, protocol::{bodies::{general_request::GeneralRequest, general_response::GeneralResponse, tag_empty_body::EmptyBody}, command_id::CalIDs, CommunicationProtocolIDs, ProtocolError}, rwlock::RwLock};

pub struct CalibrationSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> CalibrationSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> CalibrationControl for CalibrationSerialControl<'a, T> {
    fn set_angle_sensor_static_error(
        &mut self,
        rear_arm_angle_error: f32,
        front_arm_angle_error: f32,
    ) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut request_buffer = [0u8;8];
        request_buffer[..4].copy_from_slice(&rear_arm_angle_error.to_le_bytes());
        request_buffer[4..8].copy_from_slice(&front_arm_angle_error.to_le_bytes());
        let request = GeneralRequest {  params: &request_buffer};
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::Cal(CalIDs::AngleSensorStaticError), request, write=true)?;
        Ok(())
    }

    fn get_angle_sensor_static_error(&mut self) -> Result<(f32, f32), DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8;8];
        let response = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::Cal(CalIDs::AngleSensorStaticError), &mut response_buffer)?;
        if response.params.len() < 8 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }

        let rear_angle_buffer: [u8; 4] = match (&response.params[..4]).try_into() {
            Ok(x) => x,
            Err(_) => return Err(DobotError::Protocol(ProtocolError::BufferTooSmall)),
        };
        let front_angle_buffer: [u8; 4] = match (&response.params[4..8]).try_into() {
            Ok(x) => x,
            Err(_) => return Err(DobotError::Protocol(ProtocolError::BufferTooSmall)),
        };
        let rear_angle = f32::from_le_bytes(rear_angle_buffer);
        let front_angle = f32::from_le_bytes(front_angle_buffer);

        Ok((rear_angle, front_angle))
    }
}
