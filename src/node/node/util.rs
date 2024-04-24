

pub fn new_current_thread_tokio_rt() -> TokioRuntime {
    let rt = tokio::runtime::Builder::new_current_thread()
    .enable_time().enable_io()
    .build().unwrap();
    rt
}