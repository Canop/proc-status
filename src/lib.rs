//!
//! basic process information
//!
//! The data comes from `/proc/<pid>/process` and is only
//! available on unix-like systems.
//!
//! This crate aims at keeping very simple.
//! If it doesn't cover your needs, you should probably have a look
//! at the much more complete [procfs](https://crates.io/crates/procfs).
//!
//! # Examples:
//!
//! ## Dump memory info about the current process:
//!
//! ```
//! let mem = proc_status::mem_usage().unwrap();
//! println!("Mem usage in bytes: current={}, peak={}", mem.current, mem.peak);
//! ```
//! This prints something like
//!
//! ```stdout
//! Mem usage in bytes: current=1232896, peak=141430784
//! ```
//!
//!
//! ## Print all the fields of the current process' status:
//!
//! ```
//! use proc_status::ProcStatus;
//!
//! let ps = ProcStatus::read().unwrap();
//! for entry in ps.entries() {
//!     let entry = entry.unwrap();
//!     println!("{} = {:?}", entry.key, entry.value);
//! }
//! ```
//!
//! ## Get the raw value of specific entries
//!
//! ```
//! use proc_status::ProcStatus;
//!
//! let ps = ProcStatus::read().unwrap();
//! println!("State: {:?}", ps.value("State").unwrap());
//! println!("VmPeak in bytes: {:?}", ps.value_KiB("VmPeak").unwrap() * 1024);
//! ```
//!


mod entry;
mod entries;
mod errors;
mod mem_usage;
mod proc_ref;
mod status;

pub use {
    entry::ProcEntry,
    entries::ProcEntries,
    errors::ProcStatusError,
    mem_usage::MemUsage,
    proc_ref::ProcRef,
    status::ProcStatus,
};

/// load information about the current en peak memory
/// usage of the current process
pub fn mem_usage() -> Result<MemUsage, ProcStatusError> {
    ProcStatus::read()
        .and_then(|ps| ps.mem_usage())
}

/// get the value of an entry by name
///
///
/// If you want to read several entries and you
/// want them to be consistent, you should read
/// the whole ProcStatus and call the `value`
/// function on that instance. This would also
/// be more efficient.
pub fn value(key: &str) -> Result<String, ProcStatusError> {
    ProcStatus::read()
        .and_then(|ps| ps.value(key).map(|v| v.to_string()))
}

/// get the value of an entry by name if it is
/// a size in KiB.
///
///
/// If you want to read several entries and you
/// want them to be consistent, you should read
/// the whole ProcStatus and call the `value_KiB`
/// function on that instance. This would also
/// be more efficient.
#[allow(non_snake_case)]
pub fn value_KiB(key: &str) -> Result<usize, ProcStatusError> {
    ProcStatus::read()
        .and_then(|ps| ps.value_KiB(key))
}


