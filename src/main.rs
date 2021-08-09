#![allow(missing_docs)]
#![allow(unused_variables, dead_code, unused_imports)]

#[macro_use]
extern crate log;

use env_logger::Env;

use mirror::core::{self, Config, Session};
use mirror::rays::Properties;

#[doc(hidden)]
fn main() {
    env_logger::init_from_env(Env::default().filter_or("LOGGING", "trace"));

    // Initialize core
    core::init();

    let config_filename = "scenes/alloy/config.toml";

    let config = Config::new(&Properties::load(config_filename), None);
    let mut session: Session = Session::new(&config, None, None);

    let halt_time: i64 = config.get("batch.halt_time").unwrap_or(3);
    let halt_spp: i64 = config.get("batch.halt_spp").unwrap_or(3);

    // Start the rendering
    session.start();

    //let stats = session.stats();
    while !session.done() {
        session.update_stats();

        let elapsed_time: f64 = session.stats().get("stats.engine.time").unwrap();
        let pass: i64 = session.stats().get("stats.engine.pass").unwrap();
        let convergence: f64 = session.stats().get("stats.engine.convergence").unwrap();
        let total_sample_sec: f64 = session
            .stats()
            .get("stats.engine.total.sample_sec")
            .unwrap();
        let triangle_count: f64 = session.stats().get("stats.dataset.triangle_count").unwrap();

        info!("[Elapsed time: {}/{}sec][Samples {}/{}][Convergence {}][Avg. samples/sec {} on {} tris",
              elapsed_time, halt_time, pass, halt_spp, convergence * 100.0, total_sample_sec, triangle_count
        )
    }

    // Stop the rendering
    session.stop();

    // Save the rendered image
    session.film().save_outputs();

    info!("# Successful");
}
