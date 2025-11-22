# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**DO NOT** open public issues for security vulnerabilities.

### Reporting Process

1. **Email**: Send security reports to security@universal-connector.org (if available) or create a private security advisory on GitHub
2. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)
3. **Response Time**: We aim to respond within 48 hours
4. **Disclosure**: Coordinated disclosure after patch is available (typically 90 days)

### Security Scope

**In Scope:**
- LSP server vulnerabilities (command injection, memory safety)
- HTTP API vulnerabilities (authentication bypass, injection attacks)
- WebSocket vulnerabilities (message injection, DoS)
- Dependency vulnerabilities
- Build process security issues

**Out of Scope:**
- Editor client vulnerabilities (responsibility of editor maintainers)
- Denial of service requiring unrealistic resources
- Social engineering attacks

## Security Measures

### Current Protections

1. **Memory Safety**: Rust's ownership system prevents:
   - Buffer overflows
   - Use-after-free
   - Data races
   - Null pointer dereferences

2. **Input Validation**:
   - All HTTP inputs validated
   - LSP messages validated against protocol
   - Document size limits enforced
   - Format validation for conversions

3. **Dependency Management**:
   - Regular `cargo audit` runs
   - Minimal dependency surface
   - Pinned versions in Cargo.lock

4. **Build Security**:
   - Reproducible builds via Cargo
   - No unsafe code blocks
   - Strict compiler warnings

### Known Limitations

1. **No Authentication**: Server currently has no authentication mechanism
   - **Mitigation**: Deploy behind reverse proxy with auth
   - **Status**: Planned for v0.2.0

2. **No Rate Limiting**: APIs not rate-limited
   - **Mitigation**: Use reverse proxy rate limiting
   - **Status**: Planned for v0.2.0

3. **No TLS**: Server doesn't implement TLS
   - **Mitigation**: Use reverse proxy (nginx, Apache)
   - **Status**: Recommended deployment pattern

4. **Document Storage**: In-memory only, no persistence
   - **Impact**: DoS via memory exhaustion possible
   - **Mitigation**: Resource limits at OS/container level
   - **Status**: Acceptable for current use case

## Security Best Practices

### Deployment

```bash
# Run as non-root user
useradd -r -s /bin/false connector
sudo -u connector ./universal-connector-server

# Use firewall rules
ufw allow from trusted_ip to any port 8080
ufw deny 8080

# Container security
docker run --read-only --cap-drop=ALL --security-opt=no-new-privileges \
  --memory=100m --cpus=1.0 universal-connector
```

### Configuration

```bash
# Disable unused features
export ENABLE_LSP=true
export ENABLE_HTTP=false  # If not needed
export ENABLE_WS=false    # If not needed

# Bind to localhost only
export HTTP_ADDR=127.0.0.1:8080
export WS_ADDR=127.0.0.1:8081
```

## Security Roadmap

- [ ] v0.2.0: Add JWT-based authentication
- [ ] v0.2.0: Implement rate limiting
- [ ] v0.3.0: Add TLS support
- [ ] v0.3.0: Implement document size limits
- [ ] v0.4.0: Add audit logging
- [ ] v0.4.0: Implement RBAC (Role-Based Access Control)

## Vulnerability History

No vulnerabilities have been reported or discovered as of 2025-11-22.

## Contact

- **Security Email**: security@universal-connector.org
- **PGP Key**: [To be added]
- **GitHub Security Advisories**: [Repository Security Tab]

## Acknowledgments

We appreciate responsible disclosure and will acknowledge security researchers who report vulnerabilities.

---

**Last Updated**: 2025-11-22
**Version**: 1.0
