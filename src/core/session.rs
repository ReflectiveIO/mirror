use std::collections::HashMap;
use std::convert::TryInto;

use super::config::Config;
use super::film::Film;
use super::state::State;
use crate::rays::device::IntersectionDevice;
use crate::rays::Properties;
use crate::slg::engine::tile::Tile;
use crate::slg::engine::{CPUTileRenderEngine, EngineType, TilePathCPURenderEngine};
use crate::slg::{self};

/// Session executes a rendering based on the Config provided.
#[derive(Default)]
pub struct Session {
    /// A reference to the Session
    session: slg::Session,
    /// A reference to the Config
    config: Config,
    /// Pointer to the State.
    state: State,
    /// The reference to the Film
    film: Film,
    /// A Properties container with the statistics
    stats: Properties,
}

impl Session {
    /// Create a new Session using the provided config.
    ///
    /// config is the Config used to create the rendering session.
    /// The Config is not deleted by the destructor. and optional state
    /// & film used to resume rendering.
    pub fn new(config: &Config, state: Option<State>, film: Option<Film>) -> Self {
        // @TODO: core::Session::new
        Self {
            session: Default::default(),
            config: Default::default(),
            state: Default::default(),
            film: Default::default(),
            stats: Default::default(),
        }
    }

    /// Create a new Session using the provided config and state & film.
    pub fn restart(config: &Config, state_filename: &str, film_filename: &str) -> Self {
        Session::default()
    }

    /// Returns a reference to the Config used to create this Session
    pub fn config(&self) -> &Config { &self.config }

    /// Returns a pointer to the current State, The Session must be paused.
    pub fn state(&self) -> &State { &self.state }

    /// Starts the rendering.
    pub fn start(&mut self) {
        trace!("start the rendering");
        self.session.start();

        // In order to populate the stats.* Properties
        self.update_stats();
    }

    /// Stops the rendering.
    pub fn stop(&self) {
        trace!("stop the rendering");
        self.session.stop();
    }

    /// It can be used to check if the session has been started.
    pub fn started(&self) -> bool { self.session.started() }

    /// Stops the rendering and allows to edit the scene.
    pub fn begin_edit(&self) { self.session.begin_edit(); }

    /// Ends the scene editing and start the rendering again.
    pub fn end_edit(&mut self) {
        self.session.end_edit();

        // @TODO: Invalidate the scene properties cache
        // self.config.scene().scene_properties_cache.clear();
    }

    /// It can be used to check if the session is in scene editing mode.
    pub fn editing(&self) -> bool { self.session.editing() }

    /// Pause the rendering.
    pub fn pause(&self) { self.session.pause() }

    /// Resume the rendering.
    pub fn resume(&self) { self.session.resume() }

    /// It can be used to check if the session is paused.
    pub fn paused(&self) -> bool { self.session.paused() }

    /// It can be used to check if the rendering is over.
    pub fn done(&self) -> bool { self.session.engine.done() }

    /// Used to wait for the end of the rendering.
    pub fn wait_for_done(&self) { self.session.engine.wait_for_done() }

    /// Used to wait for the next frame with real-time render engines like
    /// RealTimePathOpenCL. it does nothing with other render engines.
    pub fn wait_for_frame(&self) { self.session.engine.wait_for_frame() }

    /// Returns a reference to a Film with the output of the rendering
    pub fn film(&self) -> &Film { &self.film }

    /// Updates the statistics.
    ///
    /// This function perform also all periodic checks and tasks
    /// (Like saving the film outputs, etc.).
    pub fn update_stats(&mut self) {
        // It is not really correct to call update_stats() outside a start()/stop()
        // however it is easy to avoid any harm if it is done.
        if !self.session.started() {
            return;
        }

        // Stats update

        // film update may be required by some render engine to update
        // statistics, convergence test and more
        self.session.engine.update_film();

        self.stats.set(
            "stats.engine.total.ray.sec",
            self.session.engine.get_total_rays_sec(),
        );
        self.stats.set(
            "stats.engine.total.sample.sec",
            self.session.engine.get_total_samples_sec(),
        );
        self.stats.set(
            "stats.engine.total.sample.sec.eye",
            self.session.engine.get_total_eye_samples_sec(),
        );
        self.stats.set(
            "stats.engine.total.sample.sec.light",
            self.session.engine.get_total_light_samples_sec(),
        );
        self.stats.set(
            "stats.engine.total.sample.count",
            self.session.engine.get_total_sample_count(),
        );
        self.stats
            .set("stats.engine.pass", self.session.engine.get_pass());
        self.stats
            .set("stats.engine.pass.eye", self.session.engine.get_eye_pass());
        self.stats.set(
            "stats.engine.pass.light",
            self.session.engine.get_light_pass(),
        );
        self.stats.set(
            "stats.engine.time",
            self.session.engine.get_rendering_time(),
        );
        self.stats.set(
            "stats.engine.convergence",
            self.session.film.get_convergence(),
        );

        // Intersection devices statistics
        let devices: &Vec<Box<dyn IntersectionDevice>> =
            self.session.engine.get_intersection_devices();
        let mut names: Vec<String> = vec![];
        let mut total_perf = 0.0;
        let mut counters: HashMap<String, u32> = HashMap::new();

        for device in devices {
            // Append a device index for the case where the same device is used multiple
            // times.
            let index = *counters.get_mut(device.name()).unwrap() + 1;
            names.push(format!("{}-{}", device.name(), index));
            let prefix = format!("stats.engine.devices.{}-{}", device.name(), index);

            self.stats.set(
                &*format!("{}.type", prefix),
                device.description().get_type(),
            );
            self.stats.set(
                &*format!("{}.performance.total", prefix),
                device.get_total_performance(),
            );
            self.stats.set(
                &*format!("{}.performance.serial", prefix),
                device.get_serial_performance(),
            );
            self.stats.set(
                &*format!("{}.performance.data_parallel", prefix),
                device.get_data_parallel_performance(),
            );

            // @TODO: Hardware device memory statistics
            // if let Some(hard_device) = device.downcast_ref::<Box<dyn HardwareDevice>>() {
            //     self.stats.set(
            //         &*format!("{}.memory.total", prefix),
            //         hard_device.description().get_max_memory() as i64,
            //     );
            //     self.stats.set(
            //         &*format!("{}.memory.used", prefix),
            //         hard_device.get_used_memory() as i64,
            //     );
            // } else {
            self.stats.set(&*format!("{}.memory.total", prefix), 0);
            self.stats.set(&*format!("{}.memory.used", prefix), 0);
            // }

            total_perf += device.get_total_performance();
        }

        self.stats.set("stats.engine.devices", names);
        self.stats.set("stats.engine.performance.total", total_perf);
        self.stats.set(
            "stats.dataset.triangle_count",
            self.session.config.scene.dataset.get_total_triangle_count() as i64,
        );

        // Some engine specific statistic
        match self.session.engine.get_type() {
            EngineType::TilePathCPURenderEngine => {
                let engine: &mut TilePathCPURenderEngine =
                    &mut self.session.engine.downcast_mut().unwrap();

                self.stats.set(
                    "stats.tile-path.tiles.size.x",
                    engine.get_tile_width() as i64,
                );
                self.stats.set(
                    "stats.tile-path.tiles.size.y",
                    engine.get_tile_height() as i64,
                );

                // Pending tiles
                {
                    let mut tiles: Vec<Tile> = vec![];
                    engine.get_pending_tiles(&mut tiles);
                    self.stats.set("stats.tile-path.tiles.pending", tiles);
                }

                // Not converged tiles
                {
                    let mut tiles: Vec<Tile> = vec![];
                    engine.get_not_converged_tiles(&mut tiles);
                    self.stats.set("stats.tile-path.tiles.not-converged", tiles);
                }

                // Converged tiles
                {
                    let mut tiles: Vec<Tile> = vec![];
                    engine.get_converged_tiles(&mut tiles);
                    self.stats.set("stats.tile-path.tiles.converged", tiles);
                }
            },
            _ => {},
        };

        // Periodic save
        self.session.check_periodic_save(false);
    }

    /// Returns a list of statistics related to the ongoing rendering. The
    /// returned Properties is granted to have content only after the first call
    /// to the UpdateStats method.
    pub fn stats(&self) -> &Properties { &self.stats }

    /// Dynamic edit the definition of Config properties.
    ///
    /// props are the Properties with the definition of:
    /// film.image-pipeline(s).* (including radiance channel scales),
    /// film.outputs.*, film.width or film.height.
    pub fn parse(&mut self, props: &Properties) { self.session.parse(props); }

    /// Save all the rendering related information (the Config, Scene, State and
    /// Film) in a file for a later restart. The resume file extension must
    /// be ".rsm".
    pub fn dump(&self, filename: &str) { self.session.dump(filename); }
}
