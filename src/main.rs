mod constraints;
mod gpu;
mod meshes;
mod object;
mod primitive;
mod scene;
mod uniq_id;

use std::f32::consts::PI;

use constraints::{Axis, BetweenConstraint, Constraint, DirectConstraint};
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

fn make_scene() -> Scene {
    let max = 150;
    let o1 = Object::new(SquareMesh::new(100))
        .position(point(150, 150))
        .color(rgb(0, 0, max))
        .build();
    let o2 = Object::new(TriangleMesh::new(100))
        .position(point(350, 350))
        .rotate(PI / 4.0)
        .color(rgb(max, 0, 0))
        .build();
    let o3 = Object::new(TriangleMesh::new(50))
        .position(point(500, 500))
        .color(rgb(max, max, 0))
        .build();
    let o3_4 = Object::new(SquareMesh::new(10))
        .position(point(600, 600))
        .color(rgb(max, max, max))
        .build();
    let o4 = Object::new(TriangleMesh::new(50))
        .position(point(700, 700))
        .color(rgb(max, max, 0))
        .build();

    let con1 = DirectConstraint::new(o1.id.clone(), o2.id.clone(), Axis::X);
    let con2 = DirectConstraint::new(o2.id.clone(), o1.id.clone(), Axis::X);
    let con3 = BetweenConstraint::new(
        o3_4.id.clone(),
        o3.id.clone(),
        o4.id.clone(),
        |object, params| {
            println!("{:?}", params);
            object.rotate(params.angle);
        },
    );

    let mut scene = Scene::new();
    scene.add_objects(vec![o1, o2, o3, o3_4, o4]);
    scene.add_constraint(con1);
    scene.add_constraint(con2);
    scene.add_constraint(con3);
    scene
}
