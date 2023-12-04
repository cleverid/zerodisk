mod app;
mod constraints;
mod gpu;
mod helpers;
mod meshes;
mod object;
mod primitive;
mod scene;
mod uniq_id;

use app::{Application, Arrow};
use primitive::point;
use scene::Scene;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() {
    let mut scene = Scene::new();
    let mut app = Application::new();
    app.add_component(Arrow::new(point(100.0, 100.0), point(300.0, 100.0)));
    app.init(&mut scene);

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut gpu_state = gpu::State::new(&window, &scene).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => match gpu_state.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => gpu_state.resize(gpu_state.size),
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
                WindowEvent::MouseInput { button, state, .. } => {
                    if !matches!(button, MouseButton::Left) {
                        return;
                    }
                    let pressed = matches!(state, ElementState::Pressed);
                    scene.set_mouse_click_left(pressed);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let changed = scene.set_mouse_position((*position).into());
                    if changed {
                        scene.process();
                        gpu_state.scene_update(&scene);
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    println!("resized");
                    gpu_state.resize(*physical_size);
                }
                WindowEvent::Moved(_) => {
                    println!("moved");
                    window.request_redraw();
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    println!("scaled");
                    gpu_state.resize(**new_inner_size);
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
