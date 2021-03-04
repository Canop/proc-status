

#[derive(Clone, Copy)]
pub enum ProcRef {
    ProcSelf, // proc/self, it the current process
    ProcId(usize),
}


