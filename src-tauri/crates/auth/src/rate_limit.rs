use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use lazy_static::lazy_static;

lazy_static! {
    static ref RATE_LIMIT: Arc<Mutex<HashMap<String, Vec<Instant>> >> = 
        Arc::new(Mutex::new(HashMap::new()));
}

/// Check if a request is allowed based on the rate limit
/// 
/// # Arguments
/// * `key` - The key to track rate limiting (e.g., IP address or username)
/// * `limit` - Maximum number of requests allowed in the time window
/// * `window` - Time window in seconds
/// 
/// # Returns
/// * `bool` - `true` if the request is allowed, `false` if rate limited
pub fn check_rate_limit(key: &str, limit: usize, window: u64) -> bool {
    let mut rate_limit = match RATE_LIMIT.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            // If the mutex was poisoned, we'll try to recover
            poisoned.into_inner()
        }
    };
    
    let now = Instant::now();
    let window_duration = Duration::from_secs(window);
    
    // Get or create the entry for this key
    let timestamps = rate_limit.entry(key.to_string())
        .or_insert_with(Vec::new);
    
    // Remove timestamps outside the current window
    timestamps.retain(|&t| now.duration_since(t) <= window_duration);
    
    // Check if we're under the limit
    if timestamps.len() < limit {
        timestamps.push(now);
        true
    } else {
        false
    }
}

/// Simple rate limiting function for auth endpoints
/// 
/// This is a simplified version that uses a default limit and window
/// for authentication endpoints.
pub fn rate_limit(key: &str) -> bool {
    // Default: 5 requests per minute per key
    check_rate_limit(key, 5, 60)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_rate_limit() {
        let key = "test_key";
        let limit = 3;
        let window = 1; // 1 second window for testing
        
        // First 3 requests should pass
        assert!(check_rate_limit(key, limit, window));
        assert!(check_rate_limit(key, limit, window));
        assert!(check_rate_limit(key, limit, window));
        
        // Fourth request in the same window should fail
        assert!(!check_rate_limit(key, limit, window));
        
        // After the window passes, requests should be allowed again
        thread::sleep(Duration::from_secs(window + 1));
        assert!(check_rate_limit(key, limit, window));
    }
}
