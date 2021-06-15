#![allow(missing_docs)]
#![allow(unused_variables, dead_code, unused_imports)]

// use crate::core::Config;
// use crate::core::film::Film;
// use crate::core::Session;
// use crate::core::State;
use crate::rays::properties::{Getter, Properties};

pub mod core;
pub mod rays;
pub mod slg;

#[doc(hidden)]
fn main() {
    // Initialize core
    core::init();

    let config_filename = "./demo.cfg";
    let cmdline_props: Properties = Properties::new();
    // let state: State = State::new();
    // let film: Film = Film::new();
    //
    // let config = Config::new(Properties::load(config_filename).set(cmdline_props), None);
    //
    // let session: Session = Session::new(&config, &state, &film);
    //
    // let halt_time: i32 = config.property("batch.halttime").get().unwrap();
    // let halt_spp: i32 = config.property("batch.haltspp").get().unwrap();
    //
    // // Start the rendering
    // session.start();
    //
    // let stats = session.stats();
    // while !session.done() {
    //     session.update_stats();
    //
    //     let elapsed_time: i32 = stats.get("stats.renderengine.time").get().unwrap();
    //     let pass: i32 = stats.get("stats.renderengine.pass").get().unwrap();
    //     let convergence: f32 = stats.get("stats.renderengine.convergence").get().unwrap();
    //     let total_sample_sec: i32 = stats.get("stats.renderengine.total.samplesec").get().unwrap();
    //     let triangle_count: i32 = stats.get("stats.dataset.trianglecount").get().unwrap();
    //
    //     println!("[Elapsed time: {}/{}sec][Samples {}/{}][Convergence {}][Avg. samples/sec {} on {} tris",
    //              elapsed_time, halt_time, pass, halt_spp, 100.0 * convergence, total_sample_sec, triangle_count
    //     )
    // }
    //
    // // Stop the rendering
    // session.stop();
    //
    // // Save the rendered image
    // session.film().save_outputs();

    println!("Successful");
}
