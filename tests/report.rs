extern crate log;
extern crate env_logger;
extern crate crisp_status_reporter;

use std::thread;
use std::time::Duration;

use log::LevelFilter;
use env_logger::Builder;
use crisp_status_reporter::Reporter;

fn setup() {
    Builder::new()
        .filter(None, LevelFilter::Trace)
        .try_init()
        .ok();
}

#[test]
fn initialize_valid() {
    setup();

    Reporter::new("YOUR_TOKEN_SECRET")
      .service_id("d657b4c1-dd07-4f94-ac7a-d4c3b4b219c1")
      .node_id("5eca824b-4134-4126-982d-2c2338ecf3ab")
      .replica_id("192.168.1.10")
      .interval(Duration::from_secs(30))
      .build();
}

#[test]
#[should_panic]
fn initialize_invalid_service_id() {
    setup();

    Reporter::new("YOUR_TOKEN_SECRET")
      .node_id("5eca824b-4134-4126-982d-2c2338ecf3ab")
      .replica_id("192.168.1.10")
      .build();
}

#[test]
#[should_panic]
fn initialize_invalid_node_id() {
    setup();

    Reporter::new("YOUR_TOKEN_SECRET")
      .service_id("d657b4c1-dd07-4f94-ac7a-d4c3b4b219c1")
      .replica_id("192.168.1.10")
      .build();
}

#[test]
#[should_panic]
fn initialize_invalid_replica_id() {
    setup();

    Reporter::new("YOUR_TOKEN_SECRET")
      .service_id("d657b4c1-dd07-4f94-ac7a-d4c3b4b219c1")
      .node_id("5eca824b-4134-4126-982d-2c2338ecf3ab")
      .build();
}

#[test]
fn run_and_end_valid() {
    setup();

    let reporter = Reporter::new("YOUR_TOKEN_SECRET")
      .service_id("d657b4c1-dd07-4f94-ac7a-d4c3b4b219c1")
      .node_id("5eca824b-4134-4126-982d-2c2338ecf3ab")
      .replica_id("192.168.1.10")
      .build();

    assert_eq!(reporter.run().is_ok(), true);

    // Hold on while the first reporting request fires (in the wild)
    thread::sleep(Duration::from_secs(15));
}
