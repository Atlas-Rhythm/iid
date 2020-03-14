use once_cell::sync::Lazy;
use rand::{rngs::OsRng, Rng};
use std::time::SystemTime;

#[cfg(feature = "pl")]
use parking_lot::Mutex;
#[cfg(not(feature = "pl"))]
use std::sync::Mutex;

#[inline(always)]
fn timestamp() -> u64 {
    let now = SystemTime::now();
    let epoch = SystemTime::UNIX_EPOCH;
    now.duration_since(epoch)
        .expect("system time before unix epoch")
        .as_secs()
}

static SERIAL: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(OsRng.gen()));
#[inline(always)]
fn serial() -> u8 {
    let mut lock = {
        #[cfg(feature = "pl")]
        {
            SERIAL.lock()
        }
        #[cfg(not(feature = "pl"))]
        {
            SERIAL.lock().expect("can't lock serial")
        }
    };
    let n = *lock;
    *lock = lock.wrapping_add(1);
    n
}

pub fn gen() -> u32 {
    let timestamp = timestamp();
    let timestamp = (((timestamp & 0xffff_0000_0000_0000) >> 48) as u16)
        ^ (((timestamp & 0x0000_ffff_0000_0000) >> 32) as u16)
        ^ (((timestamp & 0x0000_0000_ffff_0000) >> 16) as u16)
        ^ ((timestamp & 0x0000_0000_0000_ffff) as u16)
        ^ OsRng.gen::<u16>();
    let serial = serial();
    let random = OsRng.gen::<u8>();
    ((timestamp as u32) << 16) | ((serial as u32) << 8) | (random as u32)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn unique() {
        let mut set = HashSet::with_capacity(256);
        for i in 0..256 {
            let id = crate::gen();
            if set.contains(&id) {
                panic!("duplicate after {} rounds", i);
            }
            set.insert(id);
        }
    }
}
