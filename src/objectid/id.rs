use std::{
    convert::TryInto,
    sync::atomic::{AtomicUsize, Ordering},
    time::SystemTime,
};


use rand::{thread_rng, Rng};
use static_init::dynamic;
use std::ops::*;

const TIMESTAMP_SIZE: usize = 4;
const PROCESS_ID_SIZE: usize = 5;
const COUNTER_SIZE: usize = 3;

const TIMESTAMP_OFFSET: usize = 0;
const PROCESS_ID_OFFSET: usize = TIMESTAMP_OFFSET + TIMESTAMP_SIZE;
const COUNTER_OFFSET: usize = PROCESS_ID_OFFSET + PROCESS_ID_SIZE;

const MAX_U24: usize = 0xFF_FFFF;

#[dynamic]
static OID_COUNTER: AtomicUsize = AtomicUsize::new(thread_rng().gen_range(0..=MAX_U24));




impl ObjectId {
    pub fn new() -> crate::Result<ObjectId> {
        let timestamp = ObjectId::gen_timestamp()?;
        let process_id = ObjectId::gen_process_id();
        let counter = ObjectId::gen_count();

        let mut buf: [u8; 12] = [0; 12];
        buf[TIMESTAMP_OFFSET..(TIMESTAMP_SIZE + TIMESTAMP_OFFSET)]
            .clone_from_slice(&timestamp[..TIMESTAMP_SIZE]);
        buf[PROCESS_ID_OFFSET..(PROCESS_ID_SIZE + PROCESS_ID_OFFSET)]
            .clone_from_slice(&process_id[..PROCESS_ID_SIZE]);
        buf[COUNTER_OFFSET..(COUNTER_SIZE + COUNTER_OFFSET)]
            .clone_from_slice(&counter[..COUNTER_SIZE]);

        Ok(ObjectId::from_bytes(buf))
    }

    pub fn hex(&self) -> String {
        hex::encode(self.id)
    }

    // Retrieves the timestamp from an [`ObjectId`].
    pub fn timestamp(&self) -> crate::Result<SystemTime> {
        let mut buf = [0; 4];
        buf.copy_from_slice(&self.id[0..4]);
        let seconds_since_epoch = u32::from_be_bytes(buf) as u64;
        let time = SystemTime::now();
        let sub_duration = time.duration_since(SystemTime::UNIX_EPOCH)?;
        let add_duration = std::time::Duration::new(seconds_since_epoch, 0);
        Ok(time.sub(sub_duration).add(add_duration))
    }

    fn gen_timestamp() -> crate::Result<[u8; 4]> {
        // will succeed until 2106 since timestamp is unsigned
        let timestamp: u32 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs().try_into()?;
        Ok(timestamp.to_be_bytes())
    }

    /// Generate a random 5-byte array.
    fn gen_process_id() -> [u8; 5] {
        #[dynamic]
        static BUF: [u8; 5] = thread_rng().gen();

        *BUF
    }

    fn from_bytes(bytes: [u8; 12]) -> Self {
        ObjectId{id: bytes}
    }

    pub fn from_hex(string: &String) -> crate::Result<Self> {
        let hex = match hex::decode(string)?.try_into(){
            Ok(value) => value,
            Err(_) => return Ok(Self::default())
        };
        Ok(Self::from_bytes(hex))
    }

    /// Gets an incremental 3-byte count.
    /// Represented in Big Endian.
    fn gen_count() -> [u8; 3] {
        let u_counter = OID_COUNTER.fetch_add(1, Ordering::SeqCst);

        // Mod result instead of OID_COUNTER to prevent threading issues.
        let u = u_counter % (MAX_U24 + 1);

        // Convert usize to writable u64, then extract the first three bytes.
        let u_int = u as u64;

        let buf = u_int.to_be_bytes();
        let buf_u24: [u8; 3] = [buf[5], buf[6], buf[7]];
        buf_u24
    }

    pub fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.hex())
    }
}

impl<'de> Deserialize<'de> for ObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ObjectIdVisitor)
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hex())
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hex())
    }
}