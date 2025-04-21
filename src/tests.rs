#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::thread;

    // Simple runtime for running async tests
    struct TestRuntime;

    impl TestRuntime {
        fn block_on<F: Future>(future: F) -> F::Output {
            let mut future = Box::pin(future);
            let waker = futures::task::noop_waker();
            let mut cx = Context::from_waker(&waker);

            loop {
                match future.as_mut().poll(&mut cx) {
                    Poll::Ready(val) => return val,
                    Poll::Pending => thread::yield_now(),
                }
            }
        }
    }

    #[test]
    fn test_pin_json() {
        let api_key = env::var("PINATA_API_KEY").unwrap();
        let secret_key = env::var("PINATA_SECRET_KEY").unwrap();

        let client = PinataClient::new(api_key, secret_key);

        let json_data = serde_json::json!({
            "name": "test",
            "value": 42
        });

        let result = TestRuntime::block_on(client.pin_json(&json_data));
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.ipfs_hash.is_empty());
        assert!(response.pin_size > 0);
        assert!(!response.timestamp.is_empty());
    }

    #[test]
    fn test_pin_file() {
        let api_key = env::var("PINATA_API_KEY").unwrap();
        let secret_key = env::var("PINATA_SECRET_KEY").unwrap();

        let client = PinataClient::new(api_key, secret_key);

        // Create a temporary test file
        let temp_file = "test.txt";
        std::fs::write(temp_file, "Test content").unwrap();

        let result = TestRuntime::block_on(client.pin_file(temp_file));
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.ipfs_hash.is_empty());
        assert!(response.pin_size > 0);
        assert!(!response.timestamp.is_empty());

        // Clean up
        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_unpin() {
        let api_key = env::var("PINATA_API_KEY").unwrap();
        let secret_key = env::var("PINATA_SECRET_KEY").unwrap();

        let client = PinataClient::new(api_key, secret_key);

        // First pin something to unpin
        let json_data = serde_json::json!({
            "name": "test",
            "value": 42
        });

        let pin_result = TestRuntime::block_on(client.pin_json(&json_data)).unwrap();

        // Now try to unpin it
        let result = TestRuntime::block_on(client.unpin(&pin_result.ipfs_hash));
        assert!(result.is_ok());
    }
}
