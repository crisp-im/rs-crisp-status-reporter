# rs-crisp-status-reporter

[![Build Status](https://img.shields.io/travis/crisp-im/rs-crisp-status-reporter/master.svg)](https://travis-ci.org/crisp-im/rs-crisp-status-reporter) [![Dependency Status](https://deps.rs/repo/github/crisp-im/rs-crisp-status-reporter/status.svg)](https://deps.rs/repo/github/crisp-im/rs-crisp-status-reporter)

* [Documentation](https://docs.rs/crate/crisp-status-reporter)
* [Crate](https://crates.io/crates/crisp-status-reporter)

**Crisp Status Reporter for Rust.**

Crisp Status Reporter is used to actively submit health information to Crisp Status from your apps. Apps are best monitored via application probes, which are able to report detailed system information such as CPU and RAM load. This lets Crisp Status show if an application host system is under high load.

## How to install?

Include `crisp-status-reporter` in your `Cargo.toml` dependencies:

```toml
[dependencies]
crisp-status-reporter = "1.0"
```

## How to use?

### Create reporter

`crisp-status-reporter` can be instantiated as such:

```rust
extern crate crisp_status_reporter;

use std::time::Duration;
use crisp_status_reporter::Reporter;

// Build reporter
let reporter = Reporter::new("YOUR_TOKEN_SECRET")
  .probe_id("relay")                  // Probe ID containing the parent Node for Replica
  .node_id("socket-client")           // Node ID containing Replica
  .replica_id("192.168.1.10")         // Unique Replica ID for instance (ie. your IP on the LAN)
  .interval(Duration::from_secs(30))  // Reporting interval (in seconds; defaults to 30 seconds if not set)
  .build();

// Run reporter (starts reporting)
reporter.run();
```
