use sub_command_dobot::{
    alarm_control::AlarmSerialControl, arc_control::ArcSerialControl,
    calibration_control::CalibrationSerialControl, cp_control::CPSerialControl,
    device_control::DeviceSerialControl, eio_control::IOSerialControl,
    end_effector_control::EndEffectorSerialControl,
    handhold_teaching_control::HandholdTeachingSerialControl, home_control::HomeSerialControl,
    jog_control::JOGSerialControl, losing_step_detection_control::LosingStepDetectionSerialControl,
    ptp_control::PTPSerialControl, queue_control::QueueSerialControl,
    real_time_control::RealTimePoseSerialControl, trigger_control::TriggerSerialControl,
    wait_control::WaitSerialControl, wifi_control::WifiSerialControl,
};

use super::dobot_trait::dobot_core::{
    command_sender::{CommandSender, Dobot},
    sub_command_dobot::device_control::DeviceControl,
};
use crate::dobot::dobot_trait::rwlock::RwLock;

pub mod sub_command_dobot;

#[cfg(feature = "std")]
pub mod command_sender_serial;

pub struct DobotApiSerialController<'a, T: CommandSender> {
    command_sender: &'a RwLock<Dobot<T>>,
    pub device_controller: DeviceSerialControl<'a, T>,
    pub realtime_controller: RealTimePoseSerialControl<'a, T>,
    pub alarm_controller: AlarmSerialControl<'a, T>,
    pub homing_controller: HomeSerialControl<'a, T>,
    pub handhold_teaching_controller: HandholdTeachingSerialControl<'a, T>,
    pub end_effector_controller: EndEffectorSerialControl<'a, T>,
    pub jog_controller: JOGSerialControl<'a, T>,
    pub ptp_controller: PTPSerialControl<'a, T>,
    pub cp_controller: CPSerialControl<'a, T>,
    pub arc_controller: ArcSerialControl<'a, T>,
    pub wait_controller: WaitSerialControl<'a, T>,
    pub trigger_controller: TriggerSerialControl<'a, T>,
    pub io_controller: IOSerialControl<'a, T>,
    pub calibration_controller: CalibrationSerialControl<'a, T>,
    pub wifi_controller: WifiSerialControl<'a, T>,
    pub losing_step_controller: LosingStepDetectionSerialControl<'a, T>,
    pub queue_controller: QueueSerialControl<'a, T>,
}

impl<'a, T: CommandSender> DobotApiSerialController<'a, T> {
    pub fn new(command_sender: &'a RwLock<Dobot<T>>) -> Self {
        let device_controller = DeviceSerialControl::new(command_sender);
        let realtime_controller = RealTimePoseSerialControl::new(command_sender);
        let alarm_controller = AlarmSerialControl::new(command_sender);
        let homing_controller = HomeSerialControl::new(command_sender);
        let handhold_teaching_controller = HandholdTeachingSerialControl::new(command_sender);
        let end_effector_controller = EndEffectorSerialControl::new(command_sender);
        let jog_controller = JOGSerialControl::new(command_sender);
        let ptp_controller = PTPSerialControl::new(command_sender);
        let cp_controller = CPSerialControl::new(command_sender);
        let arc_controller = ArcSerialControl::new(command_sender);
        let wait_controller = WaitSerialControl::new(command_sender);
        let trigger_controller = TriggerSerialControl::new(command_sender);
        let io_controller = IOSerialControl::new(command_sender);
        let calibration_controller = CalibrationSerialControl::new(command_sender);
        let wifi_controller = WifiSerialControl::new(command_sender);
        let losing_step_controller = LosingStepDetectionSerialControl::new(command_sender);
        let queue_controller = QueueSerialControl::new(command_sender);

        Self {
            command_sender,
            device_controller,
            realtime_controller,
            alarm_controller,
            homing_controller,
            handhold_teaching_controller,
            end_effector_controller,
            jog_controller,
            ptp_controller,
            cp_controller,
            arc_controller,
            wait_controller,
            trigger_controller,
            io_controller,
            calibration_controller,
            wifi_controller,
            losing_step_controller,
            queue_controller,
        }
    }
}
