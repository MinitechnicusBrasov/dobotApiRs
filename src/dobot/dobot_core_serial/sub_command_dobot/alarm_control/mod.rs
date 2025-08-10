#[cfg(feature = "std")]
mod test;

use critical_section::Mutex;

use crate::dobot::dobot_trait::{
    dobot_core::{
        command_sender::{CommandSender, Dobot}, dobot_error::DobotError,
        sub_command_dobot::alarm_control::AlarmControl,
    },
    protocol::{
        alarm::Alarm, bodies::{general_response::GeneralResponse, tag_empty_body::EmptyBody}, command_id::AlarmIDs, CommunicationProtocolIDs, ProtocolError
    }, rwlock::RwLock,
};

pub struct AlarmSerialControl<'a, T: CommandSender> {
    command_sender: &'a mut RwLock<Dobot<T>>,
}

impl<'a, T: CommandSender> AlarmSerialControl<'a, T> {
    pub fn new(command_sender: &'a mut RwLock<Dobot<T>>) -> Self {
        Self { command_sender }
    }
}

impl<'a, T: CommandSender> AlarmControl for AlarmSerialControl<'a, T> {
    fn get_active_alarms(&mut self) -> Result<[Option<Alarm>; 128], DobotError> {
        let sender = create_sender!(self.command_sender)?;
        let mut response_buffer = [0u8; 16];

        let response = send_cmd!(get sender, GeneralResponse, CommunicationProtocolIDs::Alarm(AlarmIDs::GetAlarmState), &mut response_buffer)?;


        println!("{}", response.params.len());
        if response.params.len() < 16 {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }

        let mut alarms: [Option<Alarm>; 128] = [None; 128];
        for (byte_index, &byte) in response_buffer.iter().enumerate() {
            for bit_index in 0..8 {
                let bitmask = 1 << bit_index;

                if (byte & bitmask) != 0 {
                    let global_bit_index = (byte_index * 8 + bit_index) as u8;

                    let alarm = Alarm::try_from_u8(global_bit_index)?;
                    alarms[global_bit_index as usize] = Some(alarm);
                }
            }
        }

        Ok(alarms)
    }

    fn clear_all_alarms_state(&mut self) -> Result<(), DobotError> {
        let sender = create_sender!(self.command_sender)?;

        send_cmd!(send sender, EmptyBody, CommunicationProtocolIDs::Alarm(AlarmIDs::ClearAlarmState), EmptyBody {  })?;
        Ok(())
    }
}
