pub enum AppEvent {
    CpuUpdate { cpu_count: u64, total_usage: f32 },
    GreeterResponse(String),
    Disconnected(String),
}
