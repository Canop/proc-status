

/// the reference to a process, either the symbolic "self"
/// or a process id
#[derive(Clone, Copy)]
pub enum ProcRef {
    ProcSelf, // proc/self, it the current process
    ProcId(usize),
}


