use once_cell::sync::Lazy;
use std::{
    thread,
    time::{Duration, SystemTime},
};

#[cfg(feature = "pl")]
use parking_lot::Mutex;
#[cfg(not(feature = "pl"))]
use std::sync::Mutex;

/// Locks a mutex, whether it's from std or parking_lot
macro_rules! lock {
    ($mutex:expr) => {{
        #[cfg(feature = "pl")]
        {
            $mutex.lock()
        }
        #[cfg(not(feature = "pl"))]
        {
            $mutex.lock().expect("can't lock serial")
        }
    }};
}
/// `Duration` since the Unix epoch
#[inline(always)]
fn now() -> Duration {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time before unix epoch")
}

/// Unix timestamp of the previous ID
static TIMESTAMP: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(timestamp_noblock()));
/// Current Unix timestamp
#[inline(always)]
fn timestamp_noblock() -> u64 {
    now().as_secs()
}
/// Unix timestamp, might block to ensure the generated ID is unique
#[inline(always)]
fn timestamp() -> u64 {
    let mut timestamp_lock = lock!(TIMESTAMP);
    let mut serial_lock = lock!(SERIAL);

    let mut result = timestamp_noblock();
    if result == *timestamp_lock {
        if u16::max_value() == *serial_lock {
            // Block until next second if all possible unique IDs are already generated for the current timestamp
            thread::sleep(Duration::from_nanos(
                (1_000_000_000 - now().subsec_nanos()) as u64,
            ));
            result = timestamp_noblock();

            *serial_lock = 0;
        }
    } else {
        // Reset the serial to 0 if the current timestamp is different from the one used for the previous ID
        *serial_lock = 0;
    }

    // Update the cached timestamp
    *timestamp_lock = result;
    result
}

/// Number of IDs generated for the current timestamp
static SERIAL: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(u16::min_value()));
/// Returns the current serial and increases it
#[inline(always)]
fn serial() -> u16 {
    let mut lock = lock!(SERIAL);
    let n = *lock;
    *lock += 1;
    n
}

pub fn gen() -> u64 {
    let timestamp = timestamp() & 0x0000_000f_ffff_ffff;
    let serial = serial();
    (timestamp << 16) | serial as u64
}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn unique() {
        let mut set = HashSet::with_capacity(100_000);
        for i in 0..100_000 {
            let id = crate::gen();
            if set.contains(&id) {
                panic!("duplicate after {} rounds", i);
            }
            set.insert(id);
        }
    }

    #[test]
    fn unique_concurrent() {
        let ids: Vec<u64> = [0; 100_000].par_iter().map(|_| crate::gen()).collect();
        let mut set = HashSet::with_capacity(100_000);
        for (i, id) in ids.iter().enumerate() {
            if set.contains(&id) {
                panic!("duplicate at index {}", i);
            }
            set.insert(id);
        }
    }
}
