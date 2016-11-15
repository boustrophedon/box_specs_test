extern crate specs;
extern crate libbox;
extern crate time;

fn main() {
    let timestep = time::Duration::milliseconds(1);
    let ctx = libbox::client::BClientContext::new(timestep);

    let mut game = libbox::client::ClientGame::new();

    let mut accum = timestep;
    let mut t = time::PreciseTime::now();
    while game.is_running() {
        while accum >= timestep {
            game.run(ctx.clone());
            accum = accum - timestep; 
        }
        let now = time::PreciseTime::now();
        accum = accum + t.to(now);
        t = now;
    }
    // game.do_cleanup(); // TODO
}
