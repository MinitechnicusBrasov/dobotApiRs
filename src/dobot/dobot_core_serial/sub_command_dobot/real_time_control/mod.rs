#[cfg(feature = "std")]
mod test;


use crate::dobot::dobot_trait::{dobot_core::{
        command_sender::{CommandSender, Dobot},
        dobot_error::DobotError,
        sub_command_dobot::real_time_control::RealTimeControl,
    }, protocol::{bodies::{general_request::GeneralRequest, general_response::GeneralResponse, tag_empty_body::EmptyBody, tag_pose::TagPose}, command_id::DevicePoseIDs, CommunicationProtocolIDs, ProtocolError}, rwlock::RwLock};

pub struct RealTimePoseSerialControl<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> RealTimePoseSerialControl<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> RealTimeControl for RealTimePoseSerialControl<'a, T> {
    fn reset_pose(
        &mut self,
        manual: u8,
        rear_arm_angle: f32,
        front_arm_angle: f32,
    ) -> Result<(), DobotError> {
        let mut request_buffer = [0u8; 9];
        request_buffer[0] = manual;
        request_buffer[1..5].copy_from_slice(&rear_arm_angle.to_le_bytes());
        request_buffer[5..9].copy_from_slice(&front_arm_angle.to_le_bytes());
        let request_body = GeneralRequest {
            params: &request_buffer,
        };
        let sender = create_sender!(self.command_sender)?;
        send_cmd!(send sender, GeneralRequest, CommunicationProtocolIDs::DevicePose(DevicePoseIDs::ResetPose), request_body, write=true)?;

        Ok(())
    }

    fn get_pose(&mut self) -> Result<TagPose, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 32];

        let response = send_cmd!(get sender, TagPose, CommunicationProtocolIDs::DevicePose(DevicePoseIDs::GetPose), &mut response_buffer)?;
        Ok(response)
    }

    fn get_pose_rail(&mut self) -> Result<f32, DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 4];

        let response = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::DevicePose(DevicePoseIDs::GetPoseL), &mut response_buffer)?;

        if response.params.len() < core::mem::size_of::<f32>() {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        let pose = f32::from_le_bytes([
            response.params[0],
            response.params[1],
            response.params[2],
            response.params[3],
        ]);
        Ok(pose)
    }
}
