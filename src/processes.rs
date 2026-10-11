pub enum SortBy {
    Cpu,
    Memory,
}

pub struct ProcessRow {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub memory_mb: u64,
}
