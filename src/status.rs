use {
    crate::*,
    std::{
        fs::File,
        io::Read,
    },
};

/// A snapshot of the status of a process
///
/// It's stored in a string to ensure consistency
/// and can be kept around and compared.
pub struct ProcStatus {
    content: String,
}

impl ProcStatus {

    /// read the proc status info of the current process.
    ///
    /// It's the same than `ProcStatus::read(ProcRef::ProcSelf)`
    pub fn read() -> Result<Self, ProcStatusError> {
        Self::read_for(ProcRef::ProcSelf)
    }

    /// read the proc status info of a process.
    pub fn read_for(proc_ref: ProcRef) -> Result<Self, ProcStatusError> {
        let mut file = match proc_ref {
            ProcRef::ProcSelf => File::open("/proc/self/status"),
            ProcRef::ProcId(id) => File::open(format!("/proc/{}/status", id)),
        }?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(Self { content })
    }

    /// return an iterator over all key:value entries
    pub fn entries(&self) -> ProcEntries<'_> {
        ProcEntries::from_content(&self.content)
    }

    /// find an entry by name
    pub fn entry(&self, key: &str) -> Result<ProcEntry<'_>, ProcStatusError> {
        for entry in self.entries() {
            let entry = entry?;
            if entry.key == key {
                return Ok(entry);
            }
        }
        Err(ProcStatusError::EntryNotFound(key.to_string()))
    }

    /// return the value of an entry found by key
    ///
    /// Example:
    /// ```
    /// println!(
    ///     "current process name: {:?}",
    ///     proc_status::ProcStatus::read().unwrap().value("Name").unwrap(),
    /// );
    /// ```
    /// Be careful that the values written as "xxx kB" are in
    /// KiB, not kB, and are written this way for compatibility.
    pub fn value(&self, key: &str) -> Result<&str, ProcStatusError> {
        self.entry(key).map(|e| e.value)
    }

    /// return the value of a memory related entry in KiB
    #[allow(non_snake_case)]
    pub fn value_KiB(&self, key: &str) -> Result<usize, ProcStatusError> {
        self.entry(key).and_then(|e| e.in_KiB())
    }

    /// return the current and peak ram usage of the process
    pub fn mem_usage(&self) -> Result<MemUsage, ProcStatusError> {
        self.value_KiB("VmRSS")
            .and_then(|current| {
                self.value_KiB("VmPeak")
                    .map(|peak| MemUsage {
                        current: current * 1024, // proc/status data are in KiB
                        peak: peak * 1024,
                    })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read() {
        let ps = ProcStatus::read().unwrap();
        let name = ps.value("Name").unwrap();
        println!("name: {:?}", name);
        println!("VM peak: {:?}", ps.value("VmPeak").unwrap());
        println!("VM peak KiB: {:?}", ps.entry("VmPeak").unwrap().in_KiB().unwrap());
        mem_usage().unwrap();
    }
}
