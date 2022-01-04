use bevy::{
    prelude::*,
    tasks::AsyncComputeTaskPool,
};
use crossbeam::channel::{bounded, Receiver};
use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, Default)]
pub struct ConsolePlugin;
impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_io_thread.system())
            .add_system(parse_input.system());
    }
}

fn spawn_io_thread(mut commands: Commands, thread_pool: Res<AsyncComputeTaskPool>) {
    println!("Torus debug console.  Type 'help' for list of commands.");
    print!(">>> ");
    io::stdout().flush().unwrap();

    let (tx, rx) = bounded(1);
    let task = thread_pool.spawn(async move {
        let stdin = io::stdin();
        loop {
            let line = stdin.lock().lines().next().unwrap().unwrap();
            tx.send(line)
                .expect("error sending user input to other thread");
        }
    });
    task.detach();
    commands.insert_resource(rx);
}

fn parse_input(line_channel: Res<Receiver<String>>) {
    if let Ok(line) = line_channel.try_recv() {
        let app_name = "torus";
        println!("");
        let split = line.split_whitespace();
        let mut args = vec![app_name];
        args.append(&mut split.collect());

        println!("{:?}", args);
        print!(">>> ");
        io::stdout().flush().unwrap();
    }
}
