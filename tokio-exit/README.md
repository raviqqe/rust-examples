# tokio-exit

This projects proves that `std::process::exit` is not properly handled by tokio runtime. And tasks are sometimes discarded by it with no error.
