use crate::flags::Flags;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathStatus {
    Satisfied,
    Unsatisfied,
    Satisfiable,
    Invalid,
}

impl NetworkStatus {
    pub fn get_flag<const F: u8>(&self) -> bool {
        Flags(self.flags).get_flag::<F>()
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetworkStatus {
    pub path_status: PathStatus,
    pub(crate) flags: u8,
}
impl NetworkStatus {
    pub fn flags(&self) -> Flags {
        Flags(self.flags)
    }
}
