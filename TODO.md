# TODO

## SSH Implementation Roadmap

### 1. SSH Protocol Version Exchange (RFC 4253 §4.2)
- [x] Server sends: `SSH-2.0-rssh_0.1.0\r\n`
- [x] Client sends: `SSH-2.0-rssh_0.1.0\r\n`
- [x] Parse and validate version strings

### 2. Key Exchange (KEX)
- [ ] Implement Diffie-Hellman group14-sha256
- [ ] Support RSA/Ed25519 host keys
- [ ] Generate session keys

### 3. Transport Layer
- [ ] AES-CTR encryption
- [ ] HMAC-SHA256 for integrity
- [ ] Packet format with length/padding/MAC

### 4. User Authentication
- [ ] Password authentication
- [ ] Public key authentication
- [ ] Handle auth failures/retries

### 5. Connection Protocol
- [ ] Channel multiplexing
- [ ] Session channels for shell/exec
- [ ] Window management

### 6. Basic Commands
- [ ] Shell access
- [ ] Command execution
- [ ] PTY allocation
