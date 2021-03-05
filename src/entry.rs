use {
    crate::errors::ProcStatusError,
};

/// a key:value line in the proc/status pseudo-file
///
/// key and value fields can be read directly
pub struct ProcEntry<'p> {
    pub key: &'p str,
    pub value: &'p str,
}

impl<'p> ProcEntry<'p> {
    pub(crate) fn new(src: &'p str) -> Result<Self, ProcStatusError> {
        src.find(':')
            .map(|i| Self {
                key: &src[..i],
                value: src[i+1..].trim(),
            })
            .ok_or_else(|| ProcStatusError::NoColon(src.to_string()))
    }
    /// the parsed number before 'kB', meaning the number of KiB
    /// for a memory related value
    #[allow(non_snake_case)]
    pub fn in_KiB(&self) -> Result<usize, ProcStatusError> {
        if let Some(number) = self.value.strip_suffix(" kB") {
            Ok(number.parse()?)
        } else {
            Err(ProcStatusError::NotInKib)
        }
    }
}
