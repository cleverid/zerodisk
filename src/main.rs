mod state;

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();
    let _state = state::State::new(&_window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}
