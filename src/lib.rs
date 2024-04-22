use std::future;

pub fn run() {
    let mut sigint = tokio::signal::unix::signal(unimplemented!())?;

    loop {
        if sigint.poll_recv(cx).is_ready() {
            tracing::debug!("Received SIGINT");
            unimplemented!()
        }
        unimplemented!()
    }.await;

    Ok(())
}
