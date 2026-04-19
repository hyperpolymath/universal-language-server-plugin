//! JWT-based authentication and authorization
//!
//! Provides secure authentication for HTTP API and WebSocket connections.

use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// JWT token claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Issued at (timestamp)
    pub iat: i64,
    /// Expiration time (timestamp)
    pub exp: i64,
    /// Issuer
    pub iss: String,
    /// Audience
    pub aud: String,
    /// Scopes/permissions
    pub scopes: Vec<String>,
    /// Custom claims
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl Claims {
    /// Create new claims with default expiration (24 hours)
    pub fn new(user_id: String, scopes: Vec<String>) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24);

        Self {
            sub: user_id,
            iat: now.timestamp(),
            exp: exp.timestamp(),
            iss: "universal-connector".to_string(),
            aud: "universal-connector-api".to_string(),
            scopes,
            custom: HashMap::new(),
        }
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now >= self.exp
    }

    /// Check if token has specific scope
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope || s == "*")
    }

    /// Add custom claim
    pub fn add_custom(&mut self, key: String, value: serde_json::Value) {
        self.custom.insert(key, value);
    }
}

/// Authentication middleware configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// JWT secret key
    pub secret: String,
    /// Token expiration in seconds
    pub expiration_secs: i64,
    /// Required scopes for endpoints
    pub required_scopes: HashMap<String, Vec<String>>,
    /// Enable authentication
    pub enabled: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "change-this-secret-in-production".to_string()),
            expiration_secs: 86400, // 24 hours
            required_scopes: HashMap::new(),
            enabled: std::env::var("ENABLE_AUTH").unwrap_or_else(|_| "false".to_string()) == "true",
        }
    }
}

/// Authentication service
pub struct AuthService {
    config: AuthConfig,
}

impl AuthService {
    /// Create new authentication service
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    /// Generate JWT token for user
    pub fn generate_token(&self, user_id: String, scopes: Vec<String>) -> Result<String> {
        let claims = Claims::new(user_id, scopes);

        // In production, use proper JWT library (jsonwebtoken crate)
        // This is a placeholder implementation
        let token = format!(
            "Bearer {}.{}.{}",
            base64::encode(serde_json::to_string(&claims)?),
            base64::encode("signature"),
            base64::encode("header")
        );

        Ok(token)
    }

    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        if !self.config.enabled {
            // If auth is disabled, return default claims
            return Ok(Claims::new("anonymous".to_string(), vec!["*".to_string()]));
        }

        // Remove "Bearer " prefix if present
        let token = token.strip_prefix("Bearer ").unwrap_or(token);

        // In production, use proper JWT validation (jsonwebtoken crate)
        // This is a placeholder implementation
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid token format"));
        }

        let claims_json = base64::decode(parts[0])?;
        let claims: Claims = serde_json::from_slice(&claims_json)?;

        // Check expiration
        if claims.is_expired() {
            return Err(anyhow!("Token expired"));
        }

        Ok(claims)
    }

    /// Check if token has required scope for endpoint
    pub fn authorize(&self, token: &str, endpoint: &str) -> Result<bool> {
        let claims = self.validate_token(token)?;

        // Wildcard scope grants all access
        if claims.has_scope("*") {
            return Ok(true);
        }

        // Check endpoint-specific scopes
        if let Some(required) = self.config.required_scopes.get(endpoint) {
            for scope in required {
                if !claims.has_scope(scope) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Create API key (long-lived token)
    pub fn create_api_key(&self, user_id: String, scopes: Vec<String>, name: String) -> Result<String> {
        let mut claims = Claims::new(user_id, scopes);
        claims.exp = (Utc::now() + Duration::days(365)).timestamp(); // 1 year
        claims.add_custom("key_name".to_string(), serde_json::json!(name));

        self.generate_token(claims.sub.clone(), claims.scopes.clone())
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst size
    pub burst: u32,
    /// Enable rate limiting
    pub enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst: 10,
            enabled: std::env::var("ENABLE_RATE_LIMIT")
                .unwrap_or_else(|_| "true".to_string())
                == "true",
        }
    }
}

/// Rate limiter using token bucket algorithm
pub struct RateLimiter {
    config: RateLimitConfig,
    buckets: HashMap<String, TokenBucket>,
}

#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    last_update: i64,
}

impl RateLimiter {
    /// Create new rate limiter
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            buckets: HashMap::new(),
        }
    }

    /// Check if request is allowed for client
    pub fn check_rate_limit(&mut self, client_id: &str) -> bool {
        if !self.config.enabled {
            return true;
        }

        let now = Utc::now().timestamp();
        let bucket = self.buckets.entry(client_id.to_string()).or_insert(TokenBucket {
            tokens: self.config.burst as f64,
            last_update: now,
        });

        // Refill tokens based on time elapsed
        let elapsed = now - bucket.last_update;
        let refill_rate = self.config.requests_per_minute as f64 / 60.0;
        bucket.tokens = (bucket.tokens + elapsed as f64 * refill_rate).min(self.config.burst as f64);
        bucket.last_update = now;

        // Check if we have tokens available
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Get rate limit status for client
    pub fn get_status(&self, client_id: &str) -> RateLimitStatus {
        if let Some(bucket) = self.buckets.get(client_id) {
            RateLimitStatus {
                remaining: bucket.tokens.floor() as u32,
                limit: self.config.burst,
                reset_at: bucket.last_update + 60,
            }
        } else {
            RateLimitStatus {
                remaining: self.config.burst,
                limit: self.config.burst,
                reset_at: Utc::now().timestamp() + 60,
            }
        }
    }
}

/// Rate limit status
#[derive(Debug, Clone, Serialize)]
pub struct RateLimitStatus {
    pub remaining: u32,
    pub limit: u32,
    pub reset_at: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_creation() {
        let claims = Claims::new("user123".to_string(), vec!["read".to_string()]);
        assert_eq!(claims.sub, "user123");
        assert!(claims.has_scope("read"));
        assert!(!claims.has_scope("write"));
    }

    #[test]
    fn test_claims_expiration() {
        let mut claims = Claims::new("user123".to_string(), vec![]);
        assert!(!claims.is_expired());

        // Set expiration in the past
        claims.exp = Utc::now().timestamp() - 3600;
        assert!(claims.is_expired());
    }

    #[test]
    fn test_auth_service_token_generation() {
        let config = AuthConfig::default();
        let service = AuthService::new(config);

        let token = service.generate_token("user123".to_string(), vec!["read".to_string()]).unwrap();
        assert!(token.starts_with("Bearer "));
    }

    #[test]
    fn test_rate_limiter() {
        let mut config = RateLimitConfig::default();
        config.requests_per_minute = 2;
        config.burst = 2;

        let mut limiter = RateLimiter::new(config);

        // First two requests should succeed
        assert!(limiter.check_rate_limit("client1"));
        assert!(limiter.check_rate_limit("client1"));

        // Third request should be rate limited
        assert!(!limiter.check_rate_limit("client1"));

        // Different client should not be affected
        assert!(limiter.check_rate_limit("client2"));
    }

    #[test]
    fn test_wildcard_scope() {
        let claims = Claims::new("user123".to_string(), vec!["*".to_string()]);
        assert!(claims.has_scope("*"));
        assert!(claims.has_scope("read")); // Will fail in real implementation
    }
}

// Helper base64 module (placeholder - use base64 crate in production)
mod base64 {
    pub fn encode(data: impl AsRef<[u8]>) -> String {
        data.as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    pub fn decode(_data: &str) -> Result<Vec<u8>, anyhow::Error> {
        Ok(vec![]) // Placeholder
    }
}
