use embedded_can::ExtendedId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    Accessory = 0x0B,
    Ping = 0x18,
    S88Polling = 0x10,
    S88Event = 0x11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanId {
    pub prio: u8,    // 2+2 Bit
    pub command: u8, // 8 Bit
    pub resp: u8,    // 1 Bit
    pub hash: u16,   // 16 Bit
}

pub struct S88Event {
    pub node_id: u16,
    pub address: u16,
    pub old_state: u8,
    pub new_state: u8,
    pub time: u16,
}

impl S88Event {
    pub fn as_bytes(&self) -> [u8; 8] {
        [
            (self.node_id >> 8) as u8,
            self.node_id as u8,
            (self.address >> 8) as u8,
            self.address as u8,
            self.old_state,
            self.new_state,
            (self.time >> 8) as u8,
            self.time as u8,
        ]
    }
}
pub struct Accessory {
    pub loc_id: u32,
    pub position: u8,
    pub power: u8,
    pub special_function: u16,
}

impl Accessory {
    pub fn from(data: &[u8]) -> Self {
        // copy up to 8 bytes into a fixed array, missing bytes = 0
        let mut buf = [0u8; 8];
        let n = core::cmp::min(data.len(), 8);
        buf[..n].copy_from_slice(&data[..n]);

        Self {
            loc_id: u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]),
            position: buf[4],
            power: buf[5],
            special_function: u16::from_be_bytes([buf[6], buf[7]]),
        }
    }
}

impl CanId {
    pub fn new(prio: u8, command: u8, resp: u8, hash: u16) -> Self {
        Self {
            prio,
            command,
            resp,
            hash,
        }
    }
}

impl From<CanId> for u32 {
    fn from(id: CanId) -> Self {
        ((id.prio as u32 & 0x0F) << 25)
            | ((id.command as u32 & 0xFF) << 17)
            | ((id.resp as u32 & 0x01) << 16)
            | (id.hash as u32)
    }
}

impl From<CanId> for ExtendedId {
    fn from(id: CanId) -> Self {
        ExtendedId::new(id.into()).expect("can id cannot be converted")
    }
}

impl From<u32> for CanId {
    fn from(raw: u32) -> Self {
        Self {
            prio: ((raw >> 25) & 0x0F) as u8,
            command: ((raw >> 17) & 0xFF) as u8,
            resp: ((raw >> 16) & 0x01) as u8,
            hash: (raw & 0xFFFF) as u16,
        }
    }
}

impl From<ExtendedId> for CanId {
    fn from(id: ExtendedId) -> Self {
        Self::from(id.as_raw())
    }
}

impl core::fmt::Display for CanId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "prio:{} command:{} reps:{} hash:{}",
            self.prio, self.command, self.resp, self.hash
        )
    }
}
