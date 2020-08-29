//! ECS system splits our logic into three main modules:
//! * entities: predators and prey
//! * components
//! * resources
//!
//! TODO: There's also net modules for UDP communication with actors that
//! control predator entities.

#[macro_use]
extern crate shrinkwraprs;

mod components;
pub mod conf;
mod entities;
mod prelude;
mod resources;

use bevy::{
    prelude::*,
    render::{
        camera::{ActiveCameras, Camera},
        pass::*,
        render_graph::{
            base::MainPass, CameraNode, PassNode, RenderGraph,
            WindowSwapChainNode, WindowTextureNode,
        },
        texture::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsage,
        },
    },
    window::{CreateWindow, WindowDescriptor, WindowId},
};

struct TestWindow(bevy::window::Window);

fn main() {
    let window = TestWindow(bevy::window::Window::new(
        bevy::window::WindowId::new(),
        &Default::default(),
    ));

    App::build()
        .add_resource(ClearColor(Color::rgb(0.8, 0.8, 0.8)))
        // We only do update to the prey velocity every N ms to avoid needless
        // expensive computation.
        .add_resource(resources::FlockUpdateTimer::default())
        .add_resource(window)
        .add_default_plugins()
        .add_startup_system(camera.system())
        .add_startup_system(entities::predator::init.system())
        .add_startup_system(entities::prey::init.system())
        // Must be called before any state updates.
        .add_system(entities::predator::reset_world_view.system())
        // Simulates interactions between prey and predators.
        .add_system(entities::interact.system())
        // Simulates flocking behavior for prey which isn't in danger. We should
        // run the logic which lets prey spot a predator before this system to
        // avoid needless computation.
        .add_system(entities::prey::flocking_behavior.system())
        // Moves the predators which are controlled by keyboard.
        .add_system(entities::predator::keyboard_movement.system())
        // Moves all entities along their velocity vectors.
        .add_system(entities::nudge.system())
        .run();
}

// TODO: Let the viewer choose which predator to focus on or every
// 10s change predator focus. Alternatively create a window for each.
fn camera(
    mut commands: Commands,
    mut create_window_events: ResMut<Events<CreateWindow>>,
    mut active_cameras: ResMut<ActiveCameras>,
    mut render_graph: ResMut<RenderGraph>,
    msaa: Res<Msaa>,
) {
    for node in render_graph.iter_nodes() {
        println!("Render graph node before: {:?}", node);
    }

    // let window_id = WindowId::new();
    // create_window_events.send(CreateWindow {
    //     id: window_id,
    //     descriptor: WindowDescriptor {
    //         title: "second window".to_string(),
    //         ..Default::default()
    //     },
    // });

    // add a swapchain node for our new window
    // render_graph.add_node("swapchain", WindowSwapChainNode::new(window_id));

    // add a new depth texture node for our new window
    //render_graph.add_node(
    //    "second_window_depth_texture",
    //    WindowTextureNode::new(
    //        window_id,
    //        TextureDescriptor {
    //            format: TextureFormat::Depth32Float,
    //            usage: TextureUsage::OUTPUT_ATTACHMENT,
    //            sample_count: msaa.samples,
    //            ..Default::default()
    //        },
    //    ),
    //);

    // add a new camera node for our new window
    render_graph
        .add_system_node("secondary_camera", CameraNode::new("Secondary"));

    // add a new render pass for our new window / camera
    // let mut second_window_pass = PassNode::<&MainPass>::new(PassDescriptor {
    //     color_attachments: vec![msaa.color_attachment_descriptor(
    //         TextureAttachment::Input("color_attachment".to_string()),
    //         TextureAttachment::Input("color_resolve_target".to_string()),
    //         Operations {
    //             load: LoadOp::Clear(Color::rgb(0.1, 0.1, 0.3)),
    //             store: true,
    //         },
    //     )],
    //     depth_stencil_attachment: Some(
    //         RenderPassDepthStencilAttachmentDescriptor {
    //             attachment: TextureAttachment::Input("depth".to_string()),
    //             depth_ops: Some(Operations {
    //                 load: LoadOp::Clear(1.0),
    //                 store: true,
    //             }),
    //             stencil_ops: None,
    //         },
    //     ),
    //     sample_count: msaa.samples,
    // });
    let mut main_pass: &mut PassNode<&MainPass> =
       render_graph.get_node_mut("main_pass").unwrap();

    main_pass.add_camera("Secondary");
    active_cameras.add("Secondary");
    // render_graph.add_node("second_pass", second_window_pass);

    // render_graph
    //     .add_slot_edge(
    //         "swapchain",
    //         WindowSwapChainNode::OUT_TEXTURE,
    //         "second_pass",
    //         if msaa.samples > 1 {
    //             "color_resolve_target"
    //         } else {
    //             "color_attachment"
    //         },
    //     )
    //     .expect("Cannot add swapchain slot to second pass");

    // render_graph
    //     .add_slot_edge(
    //         "main_pass_depth_texture",
    //         WindowTextureNode::OUT_TEXTURE,
    //         "second_pass",
    //         "depth",
    //     )
    //     .expect("Cannot add slot edge from main depth texture to second pass");

    // render_graph
    //     .add_node_edge("secondary_camera", "second_pass")
    //     .expect("Cannot add second camera to second pass");

    for node in render_graph.iter_nodes() {
        println!("Render graph node: {:?}", node);
    }

    commands
        .spawn(Camera2dComponents {
            camera: bevy::render::camera::Camera {
                // window: window_id,
                name: Some("Secondary".to_string()),
                ..Default::default()
            },
            translation: Translation::new(
                conf::MAP_SIZE as f32 / 2.0,
                conf::MAP_SIZE as f32 / 2.0,
                0.0,
            ),
            // Let the viewer zoom in and out.
            scale: 1f32.into(),
            ..Default::default()
        })
        .spawn(Camera2dComponents {
            translation: Translation::new(
                conf::MAP_SIZE as f32 / 2.0,
                conf::MAP_SIZE as f32 / 2.0,
                0.0,
            ),
            // Let the viewer zoom in and out.
            scale: 1.5f32.into(),
            ..Default::default()
        });
}
