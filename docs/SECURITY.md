# Fusabi Security Policy

## ⚠️ IMPORTANT SECURITY NOTICE

**Fusabi currently does not implement sandboxing or resource limits.** This document outlines the current security status, risks, and our roadmap for improving security.

## Table of Contents

1. [Current Security Status](#current-security-status)
2. [Known Risks](#known-risks)
3. [Mitigation Strategies](#mitigation-strategies)
4. [Security Best Practices](#security-best-practices)
5. [Future Security Roadmap](#future-security-roadmap)
6. [Vulnerability Reporting](#vulnerability-reporting)
7. [Security Checklist](#security-checklist)

## Current Security Status

### What Fusabi DOES NOT Currently Provide

❌ **No Sandboxing**: Scripts have full access to host functions
❌ **No Resource Limits**: No memory or CPU usage restrictions
❌ **No Timeout Protection**: Scripts can run indefinitely
❌ **No File System Isolation**: Host functions can access any files
❌ **No Network Isolation**: Host functions can make network requests
❌ **No Process Isolation**: Scripts run in the same process as the host

### What Fusabi DOES Provide

✅ **Memory Safety**: Written in Rust with memory safety guarantees
✅ **Type Safety**: Strong static typing prevents many errors
✅ **Controlled Host Interface**: Only explicitly registered functions are accessible
✅ **No Direct Memory Access**: Scripts cannot manipulate raw pointers
✅ **Stack Overflow Protection**: Rust's stack guard pages prevent crashes

## Known Risks

### 1. Resource Exhaustion

**Risk**: Malicious or poorly written scripts can consume unlimited resources.

```fsharp
// Example: Infinite recursion
let rec infinite x = infinite x
infinite 0  // Stack overflow

// Example: Memory bomb
let rec bomb n =
    let big = Array.create 1000000 0
    bomb (n + 1)
bomb 0  // Out of memory
```

**Impact**:
- System becomes unresponsive
- Out of memory errors
- Denial of service

### 2. Infinite Loops

**Risk**: Scripts can loop forever without yielding control.

```fsharp
// Example: Infinite loop
let rec loop () = loop ()
loop ()  // Never terminates
```

**Impact**:
- CPU core at 100% usage
- Application hangs
- Requires process termination

### 3. Host Function Abuse

**Risk**: If dangerous host functions are registered, scripts can abuse them.

```rust
// DANGEROUS: Don't expose functions like this
registry.register("deleteFile", |args| {
    std::fs::remove_file(&args[0].as_str().unwrap())?;
    Ok(Value::Unit)
});
```

**Impact**:
- File system manipulation
- Data exfiltration
- System compromise

### 4. Stack Overflow

**Risk**: Deep recursion can overflow the call stack.

```fsharp
// Example: Deep recursion
let rec sum n =
    if n = 0 then 0
    else n + sum (n - 1)
sum 1000000  // Stack overflow
```

**Impact**:
- Process crash
- Potential security vulnerabilities

## Mitigation Strategies

### For Production Use

#### 1. Container Isolation

Run Fusabi in a containerized environment:

```dockerfile
FROM rust:slim
WORKDIR /app
COPY fusabi /usr/local/bin/

# Run as non-root user
RUN useradd -m -s /bin/bash fusabi
USER fusabi

# Set resource limits
CMD ["fusabi", "run", "script.fsx"]
```

Docker resource limits:
```bash
docker run --memory="100m" --cpus="0.5" fusabi-container
```

#### 2. Process-Level Limits

Use OS-level resource controls:

```bash
# Linux: Set memory limit (100MB)
ulimit -v 102400

# Set CPU time limit (10 seconds)
ulimit -t 10

# Set max file size (1MB)
ulimit -f 1024

# Run with limits
./fusabi run untrusted.fsx
```

#### 3. Timeout Wrapper

Implement external timeout control:

```rust
use std::process::Command;
use std::time::Duration;

fn run_with_timeout(script: &str, timeout: Duration) -> Result<(), Error> {
    let mut child = Command::new("fusabi")
        .arg("run")
        .arg(script)
        .spawn()?;

    match child.wait_timeout(timeout)? {
        Some(status) => Ok(()),
        None => {
            child.kill()?;
            Err("Script timeout")
        }
    }
}
```

#### 4. Restricted Host Functions

Only register safe, necessary host functions:

```rust
impl HostRegistry {
    pub fn new_restricted() -> Self {
        let mut registry = Self::new();

        // Only safe, read-only functions
        registry.register("print", safe_print);
        registry.register("getTime", safe_get_time);

        // DON'T register:
        // - File system operations
        // - Network operations
        // - Process spawning
        // - System calls

        registry
    }
}
```

### For Development Use

For trusted scripts in development:

1. **Monitor resource usage**: Use system monitoring tools
2. **Set development limits**: Configure reasonable timeouts
3. **Use version control**: Track all script changes
4. **Regular backups**: Protect against accidental damage

## Security Best Practices

### Script Development

1. **Avoid unbounded recursion**: Use accumulators or iteration
2. **Validate input sizes**: Check before allocating large structures
3. **Use tail recursion**: When supported (future feature)
4. **Test with limits**: Develop with resource constraints

### Host Application Development

1. **Principle of Least Privilege**: Only expose necessary functions
2. **Input Validation**: Validate all arguments to host functions
3. **Fail Securely**: Return errors rather than panicking
4. **Audit Host Functions**: Review all registered functions
5. **Use Capability Tokens**: Implement permission system

Example secure host function:

```rust
fn safe_read_file(args: Vec<Value>) -> Result<Value, String> {
    // Validate argument count
    if args.len() != 1 {
        return Err("Expected 1 argument".into());
    }

    // Validate path
    let path = args[0].as_str()
        .ok_or("Path must be string")?;

    // Restrict to safe directory
    if !path.starts_with("/safe/") {
        return Err("Access denied".into());
    }

    // Size limit
    let metadata = std::fs::metadata(path)
        .map_err(|_| "File not found")?;
    if metadata.len() > 1_000_000 {
        return Err("File too large".into());
    }

    // Read with error handling
    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("Read error: {}", e))?;

    Ok(Value::Str(contents))
}
```

## Future Security Roadmap

### Phase 1: Resource Limits (Q2 2024)

- [ ] Memory allocation limits per VM
- [ ] Stack depth limits
- [ ] Instruction count limits
- [ ] Array size limits

```rust
let mut vm = Vm::with_limits(Limits {
    max_memory: 100 * 1024 * 1024,  // 100MB
    max_stack_depth: 1000,
    max_instructions: 1_000_000,
    max_array_size: 10_000,
});
```

### Phase 2: Execution Control (Q3 2024)

- [ ] Async cancellation tokens
- [ ] Execution timeout
- [ ] Yield points for cooperation
- [ ] Interrupt handling

```rust
let mut vm = Vm::new();
let handle = vm.execute_async(script);

// Cancel after 5 seconds
tokio::time::timeout(Duration::from_secs(5), handle).await?;
```

### Phase 3: Sandboxing (Q4 2024)

- [ ] Capability-based security
- [ ] File system virtualization
- [ ] Network access control
- [ ] Process isolation

```rust
let sandbox = Sandbox::new()
    .allow_read("/app/data")
    .deny_network()
    .max_memory(50_000_000);

let mut vm = Vm::with_sandbox(sandbox);
```

### Phase 4: Advanced Security (2025)

- [ ] WebAssembly compilation target
- [ ] Formal verification of VM
- [ ] Security audit
- [ ] Common Criteria certification

## Vulnerability Reporting

### Reporting Process

If you discover a security vulnerability in Fusabi:

1. **DO NOT** create a public issue
2. **Email**: security@fusabi.dev
3. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### Response Timeline

- **24 hours**: Initial acknowledgment
- **72 hours**: Preliminary assessment
- **7 days**: Fix development begins
- **30 days**: Fix released (critical issues faster)

### Security Advisories

Published at: https://github.com/fusabi-lang/fusabi/security/advisories

## Security Checklist

### Before Deploying Fusabi

- [ ] Understand the security limitations
- [ ] Implement external resource limits
- [ ] Use container/VM isolation
- [ ] Restrict host function access
- [ ] Implement timeout controls
- [ ] Monitor resource usage
- [ ] Have incident response plan
- [ ] Regular security updates
- [ ] Audit scripts before running
- [ ] Backup critical data

### For Script Authors

- [ ] Test with resource limits
- [ ] Avoid infinite loops
- [ ] Validate input data
- [ ] Handle errors gracefully
- [ ] Document resource requirements
- [ ] Use efficient algorithms
- [ ] Avoid deep recursion
- [ ] Clean up resources

## Disclaimer

**Fusabi is provided "as is" without warranty of any kind.** The developers are not responsible for any damage or loss resulting from the use of Fusabi. Users are responsible for implementing appropriate security measures for their use case.

## Updates

This document was last updated: November 2024

Security status and recommendations may change. Always refer to the latest version of this document and monitor security advisories.

## Contact

- Security issues: security@fusabi.dev
- General questions: GitHub Discussions
- Updates: Follow @fusabi_lang

---

**Remember**: Until sandboxing is implemented, **only run trusted scripts** and always use external security measures in production environments.