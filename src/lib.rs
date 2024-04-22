fn _() {
    // Println doesn't do it. It must be something in `tracing`?
    tracing::debug!("").await
}
