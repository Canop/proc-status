use {
    crate::*,
    std::{
        str::Split,
    },
};

/// an iterator over the entries of a procstatus.
///
/// You get it from the proc status instance.
pub struct ProcEntries<'p> {
    split: Split<'p, char>,
}

impl<'p> ProcEntries<'p> {
    pub(crate) fn from_content(content: &'p str) -> Self {
        let split = content.split('\n');
        Self { split }
    }
}

impl<'p> Iterator for ProcEntries<'p> {
    type Item = Result<ProcEntry<'p>, ProcStatusError>;
    fn next(&mut self) -> Option<Result<ProcEntry<'p>, ProcStatusError>> {
        loop {
            match self.split.next() {
                None => { return None; }
                Some(s) => {
                    if !s.is_empty() { return Some(ProcEntry::new(s)); }
                }
            }
        }
    }
}
