use super::protocol_error::ProtocolError;
use core::convert::TryFrom;

// Helper macro to define base and offset enums

macro_rules! implement_try_from_for_protocol_group {
    ($name:ident, $( $variant:ident ),*) => { // <--- Needs to accept the variants
        impl TryFrom<u8> for $name {
            type Error = ProtocolError; // Assuming ProtocolError is defined

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                if value >= Self::min_value() && value <= Self::max_value() {
                    Ok(unsafe { core::mem::transmute::<u8, $name>(value) })
                } else {
                    Err(ProtocolError::InvalidCommandID(value)) // Adjust error variant if needed
                }
            }
        }

        impl $name {
            const fn min_value() -> u8 {
                let mut min = u8::MAX;
                $(
                    if (Self::$variant as u8) < min {
                        min = Self::$variant as u8;
                    }
                )*
                min
            }

            const fn max_value() -> u8 {
                let mut max = u8::MIN;
                $(
                    if (Self::$variant as u8) > max {
                        max = Self::$variant as u8;
                    }
                )*
                max
            }
        }
    };
}

macro_rules! define_protocol_group {
    // Add `$main_variant:ident` as the first argument
    ($main_variant:ident, $name:ident, $base:expr, { $($variant:ident = $offset:expr,)* }) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u8)]
        pub enum $name {
            $($variant = $base + $offset,)*
        }

        impl From<$name> for CommunicationProtocolIDs {
            fn from(id: $name) -> Self {
                // Use the explicit variant name instead of `paste!`
                Self::$main_variant(id)
            }
        }

        // The rest of the macro remains the same
        implement_try_from_for_protocol_group!($name, $($variant),*);
    };
}

// INFO
define_protocol_group! {
    DeviceInfo, DeviceInfoIDs, 0, {
        Sn = 0,
        Name = 1,
        Version = 2,
        WithRail = 3,
        Time = 4,
        Id = 5,
    }
}

// POSE
define_protocol_group! {
    DevicePose, DevicePoseIDs, 10, {
        GetPose = 0,
        ResetPose = 1,
        GetKinematics = 2,
        GetPoseL = 3,
    }
}

// ALARM
define_protocol_group! {
    Alarm, AlarmIDs, 20, {
        GetAlarmState = 0,
        ClearAlarmState = 1,
    }
}

// HOME
define_protocol_group! {
    Home, HomeIDs, 30, {
        HomeParams = 0,
        HomeCmd = 1,
        AutoLeveling = 2,
    }
}

// HHT
define_protocol_group! {
    Hht, HhtIDs, 40, {
        SetGetHhtTrigMode = 0,
        SetGetHhtTrigOutputEnabled = 1,
        GetHhtTrigOutput = 2,
    }
}

// ARM ORIENTATION
define_protocol_group! {
    ArmOrientation, ArmOrientationIDs, 50, {
        ArmOrientation = 0,
    }
}

// END EFFECTOR
define_protocol_group! {
    EndEffector, EndEffectorIDs, 60, {
        Params = 0,
        Laser = 1,
        SuctionCup = 2,
        Gripper = 3,
    }
}

// JOG
define_protocol_group! {
    Jog, JogIDs, 70, {
        JointParams = 0,
        CoordinateParams = 1,
        CommonParams = 2,
        Cmd = 3,
        LParams = 4,
    }
}

// PTP
define_protocol_group! {
    Ptp, PtpIDs, 80, {
        JointParams = 0,
        CoordinateParams = 1,
        JumpParams = 2,
        CommonParams = 3,
        Cmd = 4,
        LParams = 5,
        WithLCmd = 6,
        JumpToParams = 7,
        PoCmd = 8,
        PoWithLCmd = 9,
    }
}

// CP
define_protocol_group! {
    Cp, CpIDs, 90, {
        CpParams = 0,
        CpCmd = 1,
        CpleCmd = 2,
    }
}

// ARC
define_protocol_group! {
    Arc, ArcIDs, 100, {
        ArcParams = 0,
        ArcCmd = 1,
    }
}

// WAIT
define_protocol_group! {
    Wait, WaitIDs, 110, {
        WaitCmd = 0,
    }
}

// TRIG
define_protocol_group! {
    Trig, TrigIDs, 120, {
        TrigCmd = 0,
    }
}

// EIO
define_protocol_group! {
    Eio, EioIDs, 130, {
        IoMultiplexing = 0,
        Iodo = 1,
        IoPwm = 2,
        Iodi = 3,
        IoAdc = 4,
        Emotor = 5,
        Emotors = 6,
        ColorSensor = 7,
        IrSwitch = 8,
    }
}

// CAL
define_protocol_group! {
    Cal, CalIDs, 140, {
        AngleSensorStaticError = 0,
        AngleSensorCoef = 1,
        BaseDecoderStaticError = 2,
        RhandCalibrateValue = 3,
    }
}

// WIFI
define_protocol_group! {
    Wifi, WifiIDs, 150, {
        ConfigMode = 0,
        Ssid = 1,
        Password = 2,
        IpAddress = 3,
        Netmask = 4,
        Gateway = 5,
        Dns = 6,
        ConnectStatus = 7,
    }
}

// LOST STEP
define_protocol_group! {
    LostStep, LostStepIDs, 170, {
        SetLostStepParams = 0,
        SetLostStepCmd = 1,
    }
}

// CHECK MODELCONTENT
define_protocol_group! {
    CheckModel, CheckModelIDs, 180, {
        CheckModel = 1, // Note: Original CheckModelBase was 180, CheckModel was 181. Offset 1 for CheckModel.
    }
}

// PULSE MODE
define_protocol_group! {
    PulseMode, PulseModeIDs, 190, {
        PulseMode = 1, // Note: Original PulseModeBase was 190, PulseMode was 191. Offset 1 for PulseMode.
    }
}

// TEST MODE
define_protocol_group! {
    Test, TestIDs, 220, {
        TestUserParams = 0,
        TestPtpTime = 1,
    }
}

// QUEUED CMD
define_protocol_group! {
    QueuedCmd, QueuedCmdIDs, 240, {
        StartExec = 0,
        StopExec = 1,
        ForceStopExec = 2,
        StartDownload = 3,
        StopDownload = 4,
        Clear = 5,
        CurrentIndex = 6,
        LeftSpace = 7,
        Finish = 8,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CommunicationProtocolIDs {
    // INFO (Base = 0)
    DeviceInfo(DeviceInfoIDs),

    // POSE (Base = 10)
    DevicePose(DevicePoseIDs),

    // ALARM (Base = 20)
    Alarm(AlarmIDs),

    // HOME (Base = 30)
    Home(HomeIDs),

    // HHT (Base = 40)
    Hht(HhtIDs),

    // ARM ORIENTATION (Base = 50)
    ArmOrientation(ArmOrientationIDs),

    // END EFFECTOR (Base = 60)
    EndEffector(EndEffectorIDs),

    // JOG (Base = 70)
    Jog(JogIDs),

    // PTP (Base = 80)
    Ptp(PtpIDs),

    // CP (Base = 90)
    Cp(CpIDs),

    // ARC (Base = 100)
    Arc(ArcIDs),

    // WAIT (Base = 110)
    Wait(WaitIDs),

    // TRIG (Base = 120)
    Trig(TrigIDs),

    // EIO (Base = 130)
    Eio(EioIDs),

    // CAL (Base = 140)
    Cal(CalIDs),

    // WIFI (Base = 150)
    Wifi(WifiIDs),

    // LOST STEP (Base = 170)
    LostStep(LostStepIDs),

    // CHECK MODELCONTENT (Base = 180)
    CheckModel(CheckModelIDs),

    // PULSE MODE (Base = 190)
    PulseMode(PulseModeIDs),

    // TEST MODE (Base = 220)
    Test(TestIDs),

    // QUEUED CMD (Base = 240)
    QueuedCmd(QueuedCmdIDs),
}

impl TryFrom<u8> for CommunicationProtocolIDs {
    type Error = ProtocolError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // INFO (0-9)
            0..=9 => DeviceInfoIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // POSE (10-19)
            10..=19 => DevicePoseIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // ALARM (20-29)
            20..=29 => AlarmIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // HOME (30-39)
            30..=39 => HomeIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // HHT (40-49)
            40..=49 => HhtIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // ARM ORIENTATION (50-59)
            50..=59 => ArmOrientationIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // END EFFECTOR (60-69)
            60..=69 => EndEffectorIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // JOG (70-79)
            70..=79 => JogIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // PTP (80-89)
            80..=89 => PtpIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // CP (90-99)
            90..=99 => CpIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // ARC (100-109)
            100..=109 => ArcIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // WAIT (110-119)
            110..=119 => WaitIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // TRIG (120-129)
            120..=129 => TrigIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // EIO (130-139)
            130..=139 => EioIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // CAL (140-149)
            140..=149 => CalIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // WIFI (150-169)
            150..=169 => WifiIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // LOST STEP (170-179)
            170..=179 => LostStepIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // CHECK MODELCONTENT (180-189)
            180..=189 => CheckModelIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // PULSE MODE (190-199)
            190..=199 => PulseModeIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // TEST MODE (220-239)
            220..=239 => TestIDs::try_from(value).map(CommunicationProtocolIDs::from),
            // QUEUED CMD (240-255)
            240..=255 => QueuedCmdIDs::try_from(value).map(CommunicationProtocolIDs::from),
            _ => Err(ProtocolError::InvalidCommandID(value)),
        }
    }
}

impl From<CommunicationProtocolIDs> for u8 {
    fn from(id: CommunicationProtocolIDs) -> Self {
        match id {
            CommunicationProtocolIDs::DeviceInfo(val) => val as u8,
            CommunicationProtocolIDs::DevicePose(val) => val as u8,
            CommunicationProtocolIDs::Alarm(val) => val as u8,
            CommunicationProtocolIDs::Home(val) => val as u8,
            CommunicationProtocolIDs::Hht(val) => val as u8,
            CommunicationProtocolIDs::ArmOrientation(val) => val as u8,
            CommunicationProtocolIDs::EndEffector(val) => val as u8,
            CommunicationProtocolIDs::Jog(val) => val as u8,
            CommunicationProtocolIDs::Ptp(val) => val as u8,
            CommunicationProtocolIDs::Cp(val) => val as u8,
            CommunicationProtocolIDs::Arc(val) => val as u8,
            CommunicationProtocolIDs::Wait(val) => val as u8,
            CommunicationProtocolIDs::Trig(val) => val as u8,
            CommunicationProtocolIDs::Eio(val) => val as u8,
            CommunicationProtocolIDs::Cal(val) => val as u8,
            CommunicationProtocolIDs::Wifi(val) => val as u8,
            CommunicationProtocolIDs::LostStep(val) => val as u8,
            CommunicationProtocolIDs::CheckModel(val) => val as u8,
            CommunicationProtocolIDs::PulseMode(val) => val as u8,
            CommunicationProtocolIDs::Test(val) => val as u8,
            CommunicationProtocolIDs::QueuedCmd(val) => val as u8,
        }
    }
}
