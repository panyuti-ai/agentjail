//! Zero-trust egress proxy for agentjail (milestone 4).
//!
//! Three checks run on every outbound request:
//! 1. Domain allowlist.
//! 2. SSRF guard: block private, loopback and link-local addresses after DNS resolution.
//! 3. Secret scanning: block bodies that look like API keys or private keys.

/// Returns true when `host` exactly matches an entry in `allowlist`.
pub fn is_allowed_domain(host: &str, allowlist: &[&str]) -> bool {
    allowlist
        .iter()
        .any(|allowed| host.eq_ignore_ascii_case(allowed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowlist_matches_exact_host_only() {
        let allowlist = ["api.anthropic.com"];
        assert!(is_allowed_domain("api.anthropic.com", &allowlist));
        assert!(is_allowed_domain("API.Anthropic.com", &allowlist));
        assert!(!is_allowed_domain("evil.com", &allowlist));
        assert!(!is_allowed_domain("api.anthropic.com.evil.com", &allowlist));
    }
}
