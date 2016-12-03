extern crate specs;
extern crate libbox;
extern crate time;


fn main() {
    let cfg = libbox::server::ServerConfig::new();
    let timestep = cfg.timestep;
    let sim_rate = cfg.sim_rate;
    let system_planner = libbox::server::make_server_world(cfg);
    let mut game = libbox::server::ServerGame::new(system_planner, cfg);


    let mut dt = timestep;
    let mut t = time::PreciseTime::now();
    while game.is_running() {
        game.run(dt);
        let now = time::PreciseTime::now();
        dt = t.to(now);
        if sim_rate < dt {
            dt = sim_rate;
        }
        t = now;
    }

    // game.do_cleanup(); // TODO
}
