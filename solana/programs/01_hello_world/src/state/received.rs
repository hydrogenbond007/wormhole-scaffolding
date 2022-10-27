use anchor_lang::prelude::*;

pub const MESSAGE_MAX_LENGTH: usize = 1024;

#[account]
#[derive(Default)]
pub struct Received {
    pub batch_id: u32,
    pub wormhole_message_hash: [u8; 32],
    pub message: Vec<u8>,
}

impl Received {
    pub const MAXIMUM_SIZE: usize = 8 // discriminator
        + 4 // batch_id
        + 32 // wormhole_message_hash
        + 4 // Vec length
        + MESSAGE_MAX_LENGTH // message
    ;
}
