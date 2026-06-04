use chrono::Local;

#[derive(Clone)]
pub struct BatchSummary {
    pub message_count: usize,
    pub ok_count: usize,
    pub err_count: usize,
    pub elapsed_ms: u128,
    pub timestamp: String,
}

pub fn create_summary(
    message_count: usize,
    ok_count: usize,
    err_count: usize,
    elapsed_ms: u128,
) -> BatchSummary {
    BatchSummary {
        message_count,
        ok_count,
        err_count,
        elapsed_ms,
        timestamp: Local::now().to_rfc3339(),
    }
}

pub fn print_summary(summary: BatchSummary) {
    println!(
        "[Batch summary] Messages {:>4} | Ok {:>4} | Err {:>4} | {:>4} ms | {}",
        summary.message_count,
        summary.ok_count,
        summary.err_count,
        summary.elapsed_ms,
        summary.timestamp,
    );
}