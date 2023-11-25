mod constraints;
mod gpu;
mod helpers;
mod meshes;
mod object;
mod primitive;
mod scene;
mod uniq_id;

use constraints::{Axis, BetweenConstraint, DirectConstraint};
use meshes::{RectangleMesh, SquareMesh, TriangleMesh};
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

    let cube = Object::new(RectangleMesh::new(100, 100))
        .position(point(400.0, 400.0))
        .color(rgb(max, max, 0))
        .build();

    // Стрелка
    let arrow_start = Object::new(RectangleMesh::new(25, 8))
        .position(point(100.0, 100.0))
        .color(rgb(max, max, 0))
        .build();
    let arrow_line = Object::new(RectangleMesh::new(4, 100))
        .color(rgb(max, max, 0))
        .build();
    let arrow_target = Object::new(TriangleMesh::new(25))
        .position(point(300.0, 100.0))
        .color(rgb(max, max, 0))
        .build();
    let arrow_con1 = DirectConstraint::new(
        arrow_start.id.clone(),
        arrow_target.id.clone(),
        Axis::Y,
        true,
    );
    let arrow_con2 = DirectConstraint::new(
        arrow_target.id.clone(),
        arrow_start.id.clone(),
        Axis::Y,
        true,
    );
    let arrow_con3 = BetweenConstraint::new(
        arrow_line.id.clone(),
        arrow_start.id.clone(),
        arrow_target.id.clone(),
        |object, params| {
            object.rotate(params.angle);
            object.position(params.middle);
            object.mesh(RectangleMesh::new(4, params.distance))
        },
    );

    // Линия между двумя точками
    let line_start = Object::new(SquareMesh::new(15))
        .position(point(500.0, 500.0))
        .color(rgb(max, max, 0))
        .build();
    let line = Object::new(RectangleMesh::new(4, 100))
        .position(point(600.0, 600.0))
        .color(rgb(max, max, max))
        .build();
    let line_end = Object::new(SquareMesh::new(15))
        .position(point(700.0, 700.0))
        .color(rgb(max, max, 0))
        .build();
    let line_con1 = BetweenConstraint::new(
        line.id.clone(),
        line_start.id.clone(),
        line_end.id.clone(),
        |object, params| {
            object.rotate(params.angle);
            object.position(params.middle);
            object.mesh(RectangleMesh::new(4, params.distance))
        },
    );

    let mut scene = Scene::new();
    scene.add_objects(vec![
        arrow_start,
        arrow_line,
        arrow_target,
        line_start,
        line,
        line_end,
        cube,
    ]);
    scene.add_constraints(vec![
        Box::new(arrow_con1),
        Box::new(arrow_con2),
        Box::new(arrow_con3),
    ]);
    scene.add_constraint(line_con1);
    scene.process();

    scene
}
