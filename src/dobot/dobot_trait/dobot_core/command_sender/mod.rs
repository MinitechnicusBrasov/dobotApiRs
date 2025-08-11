use crate::dobot::dobot_trait::{
    protocol::{
        Body, CommunicationProtocolIDs, Protocol, ProtocolError,
        bodies::general_response::GeneralResponse,
    },
    rwlock::RwLock,
};

const MAX_PACKET_SIZE: usize = 256;

use core::fmt::Debug;

#[cfg(not(feature = "std"))]
struct FmtWriter<'a> {
    buffer: &'a mut [u8],
    cursor: &'a mut usize,
}

#[cfg(not(feature = "std"))]
impl<'a> FmtWriter<'a> {
    fn new(buffer: &'a mut [u8], cursor: &'a mut usize) -> Self {
        *cursor = 0;
        Self { buffer, cursor }
    }
}

#[cfg(not(feature = "std"))]
impl<'a> Write for FmtWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let new_cursor = *self.cursor + s.len();
        if new_cursor > self.buffer.len() {
            return Err(core::fmt::Error);
        }
        self.buffer[*self.cursor..new_cursor].copy_from_slice(s.as_bytes());
        *self.cursor = new_cursor;
        Ok(())
    }
}
use super::dobot_error::{DobotError, parse_poison_err};

pub trait CommandSender: Send + Sync {
    fn send_raw_packet(
        &mut self,
        request_packet: &[u8],
        response_buffer: &mut [u8],
    ) -> Result<usize, DobotError>;

    fn send_internal_command<'a, Req: Body<'a> + 'a, Resp: Body<'a>>(
        &mut self,
        id: CommunicationProtocolIDs,
        is_write: bool,
        is_queued: bool,
        request_body: Req,
        response_buffer: Option<&'a mut [u8]>,
    ) -> Result<Resp, DobotError> {
        let protocol = Protocol::new(id, is_queued, is_write, request_body);
        let mut request_buffer = [0u8; MAX_PACKET_SIZE]; // Max packet size
        let request_len = protocol
            .to_packet(&mut request_buffer)
            .map_err(DobotError::Protocol)?;

        let mut response_temp_buffer = [0u8; MAX_PACKET_SIZE];
        let response_len =
            self.send_raw_packet(&request_buffer[..request_len], &mut response_temp_buffer)?;
        if response_buffer.is_none() {
            if response_len < 6 {
                return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
            }
            return Ok(Resp::deserialize(&[0u8; 0])?);
        }

        let response_protocol =
            Protocol::<GeneralResponse>::from_packet(&response_temp_buffer[..response_len])?;
        let response_body_length = response_protocol.body.params.len();
        let response_buffer_reference = response_buffer.unwrap();
        if response_buffer_reference.len() < response_body_length {
            return Err(DobotError::Protocol(ProtocolError::BufferTooSmall));
        }
        response_buffer_reference[..response_body_length]
            .copy_from_slice(response_protocol.body.params);

        let response_body = Resp::deserialize(&response_buffer_reference[..response_body_length])?;
        Ok(response_body)
    }

    fn get_status_str(&self, buffer: &mut [u8]) -> Result<usize, DobotError>
    where
        Self: Debug;
}

pub struct Dobot<T: CommandSender> {
    // The conditional RwLock protects the CommandSender.
    command_sender: RwLock<T>,
}

impl<T: CommandSender> Dobot<T> {
    /// Creates a new Dobot instance with a given CommandSender.
    pub fn new(sender: T) -> Self {
        Self {
            command_sender: RwLock::new(sender),
        }
    }

    /// Example of acquiring a read lock to perform a read-only operation.
    #[cfg(feature = "std")]
    pub fn get_status(&self) -> Result<String, DobotError>
    where
        T: Debug,
    {
        let sender = self
            .command_sender
            .read()
            .map_err(|_| DobotError::SenderPoisoned)?;
        Ok(format!("Dobot status: {:?}", *sender))
    }

    #[cfg(not(feature = "std"))]
    pub fn get_status<'a, const N: usize>(
        &self,
        buffer: &'a mut [u8; N],
    ) -> Result<&'a str, DobotError>
    where
        T: Debug,
    {
        let sender = self.command_sender.read();

        let mut cursor = 0;
        write!(
            FmtWriter::new(&mut buffer[..], &mut cursor),
            "Dobot status: {:?}",
            *sender
        )?;

        // Safety: We've just written a valid UTF-8 string, so this is safe.
        Ok(unsafe { core::str::from_utf8_unchecked(&buffer[..cursor]) })
    }

    /// Sends a command to the Dobot and returns a deserialized response body.
    #[cfg(feature = "std")]
    pub fn send_command<'a, Req: Body<'a> + Send + 'a, Resp: Body<'a> + 'a>(
        &self,
        id: CommunicationProtocolIDs,
        is_write: bool,
        is_queued: bool,
        request_body: Req,
        response_buffer: Option<&'a mut [u8]>,
    ) -> Result<Resp, DobotError> {
        let mut sender = self
            .command_sender
            .write()
            .map_err(|_| DobotError::SenderPoisoned)?;
        sender.send_internal_command(id, is_write, is_queued, request_body, response_buffer)
    }

    /// Sends a command to the Dobot and returns a deserialized response body in a no-std environment.
    #[cfg(not(feature = "std"))]
    pub fn send_command<'a, Req: Body, Resp: Body>(
        &'a self,
        id: CommunicationProtocolIDs,
        is_read: bool,
        is_queued: bool,
        request_body: Req,
        response_buffer: Option<&'a mut [u8]>,
    ) -> Result<Resp, DobotError> {
        let mut sender = self.command_sender.write();
        sender.send_command(id, is_read, is_queued, request_body, response_buffer)
    }
}

#[macro_export]
macro_rules! create_sender {
    ($sender: expr) => {
        $crate::dobot::dobot_trait::dobot_core::dobot_error::parse_poison_err($sender.write())
    };
}

/// This macro provides a declarative way to call the `send_command` method on a
/// sender object, abstracting away the boilerplate of specifying command
/// parameters. It supports various command types, including those that require
/// a response and those that don't, as well as optional flags for write
/// operations and command queuing.
///
/// # Usage
/// The macro supports four main patterns, each designed for a specific use case.
///
/// ---
///
/// ## 1. `send`
/// Use this pattern when you want to send a command that **does not** require a
/// response from the recipient. This is typically for "fire-and-forget" actions.
///
/// **Syntax:** `send_cmd!(send <sender>, <request_type>, <id>, <request_body> [, write = <is_write>] [, queue = <is_queue>])`
///
/// ### Arguments
/// * `<sender>`: An expression that evaluates to the object responsible for sending the command.
/// * `<request_type>`: The type of the request body.
/// * `<id>`: A unique identifier for the command.
/// * `<request_body>`: The data payload of the command.
/// * `[write = <is_write>]` (optional): A boolean flag indicating a write operation. Defaults to `false`.
/// * `[queue = <is_queue>]` (optional): A boolean flag to queue the command. Defaults to `false`.
///
/// **Example:**
/// ```ignore
/// use crate::{send_cmd, CommandSender, MyRequestType, EmptyBody};
///
/// let sender: CommandSender = ...; // Assume sender is initialized
/// let request_body: MyRequestType = ...; // Assume request_body is initialized
///
/// // Send a command without a response
/// send_cmd!(send sender, MyRequestType, 101, request_body);
///
/// // Send a command with the 'write' flag set
/// send_cmd!(send sender, MyRequestType, 102, request_body, write = true);
/// ```
///
/// ---
///
/// ## 2. `get` (Full)
/// Use this pattern for the most flexible `get` command, where you specify both
/// the request type and the expected response type. This pattern is ideal when
/// the command requires a response and a specific request body.
///
/// **Syntax:** `send_cmd!(get <sender>, <request_type>, <response_type>, <id>, <request_body>, <buffer> [, write = <is_write>] [, queue = <is_queue>])`
///
/// ### Arguments
/// * `<sender>`: An expression that evaluates to the command sender object.
/// * `<request_type>`: The type of the request body.
/// * `<response_type>`: The type of the expected response.
/// * `<id>`: A unique identifier for the command.
/// * `<request_body>`: The data payload of the command.
/// * `<buffer>`: A mutable buffer to store the command's response.
/// * `[write = <is_write>]` (optional): A boolean flag for write operations. Defaults to `false`.
/// * `[queue = <is_queue>]` (optional): A boolean flag to queue the command. Defaults to `false`.
///
/// **Example:**
/// ```ignore
/// use crate::{send_cmd, CommandSender, MyRequestType, MyResponseType};
///
/// let sender: CommandSender = ...;
/// let request_body: MyRequestType = ...;
/// let mut response_buffer = None;
///
/// // Send a command expecting a response of `MyResponseType`
/// send_cmd!(get sender, MyRequestType, MyResponseType, 201, request_body, &mut response_buffer);
/// ```
///
/// ---
///
/// ## 3. `get` (Simplified)
/// A convenience pattern for `get` commands that use a default `GeneralRequest`
/// body. This is useful for commands that don't require specific parameters in
/// their request body but still expect a response.
///
/// **Syntax:** `send_cmd!(get <sender>, <response_type>, <id>, <buffer> [, write = <is_write>] [, queue = <is_queue>])`
///
/// ### Arguments
/// * `<sender>`: An expression that evaluates to the command sender object.
/// * `<response_type>`: The type of the expected response.
/// * `<id>`: A unique identifier for the command.
/// * `<buffer>`: A mutable buffer to store the command's response.
/// * `[write = <is_write>]` (optional): A boolean flag for write operations. Defaults to `false`.
/// * `[queue = <is_queue>]` (optional): A boolean flag to queue the command. Defaults to `false`.
///
/// **Example:**
/// ```ignore
/// use crate::{send_cmd, CommandSender, GeneralRequest, MyResponseType};
///
/// let sender: CommandSender = ...;
/// let mut response_buffer = None;
///
/// // Send a command using the simplified `get` pattern
/// send_cmd!(get sender, MyResponseType, 301, &mut response_buffer);
/// ```
///
/// ---
///
/// ## 4. `get_queue`
/// A specialized pattern for sending a command where the `is_queue` flag is
/// explicitly set to `true` and the expected response type is always
/// `TagQueueResponse`. This is for operations that specifically interact with a
/// command queue and need to retrieve its state.
///
/// **Syntax:** `send_cmd!(get_queue <sender>, <request_type>, <id>, <request_body>, <buffer> [, write = <is_write>])`
///
/// ### Arguments
/// * `<sender>`: An expression that evaluates to the command sender object.
/// * `<request_type>`: The type of the request body.
/// * `<id>`: A unique identifier for the command.
/// * `<request_body>`: The data payload of the command.
/// * `<buffer>`: A mutable buffer to store the command's response, which will be of type `TagQueueResponse`.
/// * `[write = <is_write>]` (optional): A boolean flag for write operations. Defaults to `false`.
///
/// **Example:**
/// ```ignore
/// use crate::{send_cmd, CommandSender, MyRequestType, TagQueueResponse};
///
/// let sender: CommandSender = ...;
/// let request_body: MyRequestType = ...;
/// let mut response_buffer = None;
///
/// // Send a command to get the state of a queue
/// send_cmd!(get_queue sender, MyRequestType, 401, request_body, &mut response_buffer);
/// ```
#[macro_export]
macro_rules! send_cmd {
    // Helper macro to handle optional boolean arguments.
    // If the argument is present, use its value; otherwise, use the default.
    (@opt_bool $default:expr, $value:expr) => {
        $value
    };
    (@opt_bool $default:expr,) => {
        $default
    };

    // Pattern 1: `send` command (no response).
    // This handles cases where no value is returned.
    // The `write` and `queue` parameters are optional and default to false.
    (send $sender:expr, $request_type:ty, $id:expr, $request_body:expr $(, write = $is_write:expr)? $(, queue = $is_queue:expr)?) => {
        $sender.send_command::<$request_type, EmptyBody>(
            $id,
            send_cmd!(@opt_bool false, $($is_write)?), // is_write
            send_cmd!(@opt_bool false, $($is_queue)?), // is_queue
            $request_body,
            None,
        )
    };

    // Pattern 2: `get` command with a specified request type, response type, and buffer.
    // This is the most flexible pattern, allowing you to specify a custom response type.
    // `write` and `queue` are optional flags.
    (get $sender:expr, $request_type:ty, $response_type:ty, $id:expr, $request_body:expr, $buffer:expr $(, write = $is_write:expr)? $(, queue = $is_queue:expr)?) => {
        $sender.send_command::<$request_type, $response_type>(
            $id,
            send_cmd!(@opt_bool false, $($is_write)?), // is_write
            send_cmd!(@opt_bool false, $($is_queue)?), // is_queue
            $request_body,
            Some($buffer),
        )
    };

    // Pattern 3: `get` command with `GeneralRequest` body and dynamic response type.
    // This is a common shortcut for requests with empty parameters.
    (get $sender:expr, $response_type:ty, $id:expr, $buffer:expr $(, write = $is_write:expr)? $(, queue = $is_queue:expr)?) => {
        $sender.send_command::<EmptyBody, $response_type>(
            $id,
            send_cmd!(@opt_bool false, $($is_write)?), // is_write
            send_cmd!(@opt_bool false, $($is_queue)?), // is_queue
            EmptyBody {},
            Some($buffer),
        )
    };

    // Pattern 4: `get_queue` command.
    // This is a specialized pattern for when `is_queue` is true, which returns a `TagQueueResponse`.
    (get_queue $sender:expr, $request_type:ty, $id:expr, $request_body:expr, $buffer:expr $(, write = $is_write:expr)?) => {
        $sender.send_command::<$request_type, TagQueue>(
            $id,
            send_cmd!(@opt_bool false, $($is_write)?), // is_write
            true, // is_queue (explicitly true for this pattern)
            $request_body,
            Some($buffer),
        )
    };
}

#[cfg(feature = "std")]
#[macro_use]
pub mod mock_command_sender;
