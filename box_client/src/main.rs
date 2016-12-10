extern crate specs;
extern crate libbox;
extern crate time;


fn main() {
    let cfg = libbox::client::ClientConfig::new();
    let timestep = cfg.timestep;
    let sim_rate = cfg.sim_rate;
    let system_planner = libbox::client::make_client_world(cfg);
    let mut game = libbox::client::ClientGame::new(system_planner, cfg);


    let mut frames = [time::Duration::milliseconds(0); 100];
    let mut i = 0;

    let mut dt = timestep;
    let mut t = time::PreciseTime::now();
    while game.is_running() {
        game.get_input();
        game.run(dt);
        game.render();
        let now = time::PreciseTime::now();
        dt = t.to(now); frames[i] = dt; i = (i+1)%100;
        if sim_rate < dt {
            dt = sim_rate;
        }
        t = now;
    }

    let mut s = time::Duration::milliseconds(0);
    for i in 0..100 {
        s = s + frames[i];
    }
    println!("{} fps over last 100 frames", 100.0 / (s.num_milliseconds() as f32 / 1000.0));
    // game.do_cleanup(); // TODO
}
