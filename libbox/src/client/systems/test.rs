use ::client::BClientContext;

use specs::{System, RunArg};


pub struct TestSystem {
    text: String,
    calls: u32,
}

impl TestSystem {
    pub fn new<S: Into<String>>(text: S) -> TestSystem {
        TestSystem {
            text: text.into(),
            calls: 0,
        }
    }
}

impl System<BClientContext> for TestSystem {
    fn run(&mut self, arg: RunArg, ctx: BClientContext) {
        let _ = arg.fetch(|_|{});
        println!("{} calls", self.calls);
        println!("{}", self.text);
        println!("dt: {}", ctx.dt);
        self.calls+=1;
    }
}


