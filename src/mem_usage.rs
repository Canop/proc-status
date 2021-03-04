

/// information about the physical RAM usage of a process
///
/// Note that the data, while given in bytes for simplicity,
/// isn't that precise:
/// - the numbers are truncated when written by the OS
/// - the proc_info crate own mem usage isn't 0
#[derive(Debug, Clone, Copy)]
pub struct MemUsage {

    /// estimation of the current physical memory used by the
    /// application, in bytes.
    ///
    /// Comes from  proc/<id>/status/VmRSS
    pub current: usize,

    /// estimation of the peak physical memory used by the
    /// application, in bytes.
    ///
    /// Comes from  proc/<id>/status/VmHWM
    pub peak: usize,

}
