// Enum defining all message types for ipc between actors
pub enum Messages {
    PADDLE_POS(u16),
    BALL_POS(u16, u16),
    USER_CMD_UP,
    USER_CMD_DOWN,
    SCORED,
}

impl Messages {
    // encodes the messages in bytes for sending through pipes
    // TODO This should all be done using serde
    fn encode(msg: Messages) {

    }

    fn decode(msg: Messages) {

    }
} 

#[cfg(test)]
mod tests{

}

