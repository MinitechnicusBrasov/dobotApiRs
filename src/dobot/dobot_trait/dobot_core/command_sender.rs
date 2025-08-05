use super::dobot_error::DobotError;

pub trait CommandSender {
    fn send_raw_packet(
        &self,
        request_packet: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<usize, DobotError>;

    fn wait_for_queued_command(&self, expected_idx: u64) -> Result<(), DobotError>;
}
