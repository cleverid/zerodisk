mod gpu;
mod meshes;
mod object;
mod primitive;
mod scene;
mod uniq_id;

use meshes::{SquareMesh, TriangleMesh};
use object::Object;
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
    let mut scene = make_scene();
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
                WindowEvent::CursorMoved { position, .. } => {
                    let ids = scene.tracer.trace((*position).into());
                    let changed = scene.mark_traced(ids);
                    if changed {
                        state.scene_update(&scene);
                    }
                }
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

fn make_scene() -> Scene {
    let max = 150;
    let o1 = Object::new(TriangleMesh::new(100))
        .position(point(150, 150))
        .color(rgb(0, 0, max))
        .build();
    let o2 = Object::new(TriangleMesh::new(100))
        .position(point(250, 250))
        .color(rgb(max, 0, 0))
        .build();
    let o3 = Object::new(SquareMesh::new(100))
        .position(point(60, 60))
        .color(rgb(max, max, max))
        .build();

    Scene::new().add(o1).add(o2).add(o3)
}
