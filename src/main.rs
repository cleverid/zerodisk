mod gpu;
mod objects;
mod primitive;
mod scene;

use objects::{SquareBuilder, TriangleBuilder};
use primitive::{point, rgb};
use scene::Scene;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let o1 = TriangleBuilder::new()
        .position(point(100, 100))
        .color(rgb(255, 255, 255))
        .build();
    let o2 = TriangleBuilder::new().position(point(200, 200)).build();
    let o3 = SquareBuilder::new().build();
    let mut scene = Scene::new();
    scene.add(o1).add(o2).add(o3);

    let mut state = gpu::State::new(&window, &scene).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => match state.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            Err(e) => eprintln!("{:?}", e),
        },
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
            ..
        } => {
            if window_id != window.id() {
                return;
            }
            match event {
                WindowEvent::Resized(physical_size) => {
                    println!("resized");
                    state.resize(*physical_size);
                }
                WindowEvent::Moved(_) => {
                    println!("moved");
                    window.request_redraw();
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    println!("scaled");
                    state.resize(**new_inner_size);
                }
                WindowEvent::CloseRequested => {
                    println!("Close by button");
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        }
        _ => (),
    });
}
