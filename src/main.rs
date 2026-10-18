use benny::plugins;
use benny::prelude::*;

struct GreeterPlugin;

impl Plugin for GreeterPlugin {
    fn name(&self) -> &str {
        "greeter"
    }

    fn tags(&self) -> &[&str] {
        &["core"]
    }

    fn init(&mut self, ctx: &mut Context) -> Result<(), EngineError> {
        ctx.set("greeting", "Hello, world!");
        Ok(())
    }

    fn run(&mut self, ctx: &mut Context) -> Result<(), EngineError> {
        if let Some(g) = ctx.get("greeting") {
            println!("{g}");
        }
        Ok(())
    }
}

struct CounterPlugin {
    count: u32,
}

impl Plugin for CounterPlugin {
    fn name(&self) -> &str {
        "counter"
    }

    fn tags(&self) -> &[&str] {
        &["core"]
    }

    fn run(&mut self, ctx: &mut Context) -> Result<(), EngineError> {
        self.count += 1;
        ctx.set("count", self.count.to_string());
        println!("count = {}", self.count);
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|a| a == "--list") {
        let mut engine = Engine::new();
        engine.register(GreeterPlugin);
        engine.register(CounterPlugin { count: 0 });
        plugins::register_all(&mut engine);
        for (name, tags) in engine.plugins_info() {
            println!("{name}: [{}]", tags.join(", "));
        }
        return;
    }

    let mut engine = Engine::new();
    engine.register(GreeterPlugin);
    engine.register(CounterPlugin { count: 0 });
    plugins::register_all(&mut engine);

    // Determine selection: tags or explicit names.
    let tags: Vec<&str> = args.iter().map(String::as_str).collect();
    let run_all = tags.is_empty();

    if run_all {
        engine.init_all().expect("init failed");
        engine.run().expect("run failed");
        engine.run().expect("run failed");
    } else {
        // Determine selected plugin names from tags or explicit names.
        let mut selected: Vec<String> = Vec::new();
        let mut is_tag = false;
        for tag in &tags {
            let matches = engine.plugins_with_tag(tag);
            if !matches.is_empty() {
                is_tag = true;
                for m in matches {
                    if !selected.iter().any(|s| s == m) {
                        selected.push(m.to_string());
                    }
                }
            }
        }
        if !is_tag {
            selected = tags.iter().map(|t| t.to_string()).collect();
        }

        engine.init_names(&selected).expect("init failed");
        engine.run_names(&selected).expect("run failed");
    }

    engine.shutdown();
}
