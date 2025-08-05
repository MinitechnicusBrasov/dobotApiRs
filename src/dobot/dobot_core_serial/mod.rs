use sub_command_dobot::device_control::DeviceSerialControl;

use super::dobot_trait::dobot_core::command_sender::CommandSender;

pub mod sub_command_dobot;

#[cfg(feature = "std")]
pub mod command_sender_serial;

pub struct DobotApiSerialController<'a, T: CommandSender> {
    command_sender: T,
    device_controller: DeviceSerialControl<'a, T>
}
