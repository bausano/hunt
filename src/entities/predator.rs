//! A predator is an entity which is controlled by an agent. When an agent
//! joins the game over a UDP socket, a new predator is added to the game.
//!
//! A predator is slower than a prey, therefore predators must cooperate with
//! each other in order to score points.
//!
//! A predator can also be controlled by keyboard for debugging purposes.

use crate::{
    components::{KeyboardControlled, Velocity},
    prelude::*,
};

use bevy::{
    render::{
        camera::{ActiveCameras, Camera},
        render_graph::RenderGraph,
    },
    window::CreateWindow,
};

pub struct Predator {
    // Lists positions of prey nearby. With each tick, this value should get
    // reset and repopulated.
    nearby_prey: Vec<Vec3>,
}

/// Predators are actors that join over UDP or keyboard actors. When a predator
/// joins a game, new window with camera focused on them is created.
/// TODO: Allow predators join over UDP and make keyboard predator optional.
pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut create_window_events: ResMut<Events<CreateWindow>>,
    mut active_cameras: ResMut<ActiveCameras>,
    mut render_graph: ResMut<RenderGraph>,
    msaa: Res<Msaa>,
) {
    let texture_handle = asset_server
        .load(conf::predator::ICON)
        .expect("Cannot load predator sprite");

    // Creates new window with a camera that always focuses on the player.
    let camera_name = camera::new(
        "keyboard_player".to_string(),
        &mut create_window_events,
        &mut active_cameras,
        &mut render_graph,
        &msaa,
    );

    commands
        .spawn(Camera2dComponents {
            camera: Camera {
                name: Some(camera_name),
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with_bundle((
            Predator::new(),
            Velocity::default(),
            Translation::random(),
            Rotation::default(),
            KeyboardControlled,
        ));
}

/// Moves those predators which are controlled by keyboard.
/// TODO: It'd be nice to have this method deleted and use only the parent once.
/// However I don't know how to check whether an entity is prey or predator.
pub fn keyboard_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut predator_query: Query<(&mut Velocity, &Predator, &KeyboardControlled)>,
) {
    for (mut vel, ..) in &mut predator_query.iter() {
        super::keyboard_movement(
            &time,
            &keyboard_input,
            &mut vel,
            conf::predator::MAX_SPEED,
        )
    }
}

/// Resets the state which is at the end of each tick sent to the actor which
/// controls the predator. This method MUST be called in the beginning of each
/// tick before any world update happens.
/// TODO: It'd be nice to have this as foreach, but bevy types are broken for now.
pub fn reset_world_view(mut predator_query: Query<&mut Predator>) {
    for mut predator in &mut predator_query.iter() {
        predator.nearby_prey.clear();
    }
}

impl Predator {
    /// Adds a new prey position into its world view.
    pub fn spot_prey(&mut self, at: Vec3) {
        self.nearby_prey.push(at);
    }

    /// TODO
    pub fn score(&mut self) {
        println!("Prey eaten!");
    }

    fn new() -> Self {
        Self {
            nearby_prey: Vec::new(),
        }
    }
}

mod camera {
    //! Each predator gets their own window with a camera that focuses on them.
    //! This helps us debug the behavior.

    use bevy::{
        render::{
            camera::ActiveCameras,
            pass::*,
            render_graph::{
                base::MainPass, CameraNode, PassNode, RenderGraph,
                WindowSwapChainNode, WindowTextureNode,
            },
            texture::{TextureDescriptor, TextureFormat, TextureUsage},
        },
        window::{CreateWindow, WindowDescriptor, WindowId},
    };

    use super::*;

    /// Creates a new window and camera which displays its content. Returns the
    /// name of the camera.
    ///
    /// Code based on https://github.com/bevyengine/bevy/blob/master/examples/window/multiple_windows.rs
    pub fn new(
        predator_name: String,
        create_window_events: &mut Events<CreateWindow>,
        active_cameras: &mut ActiveCameras,
        render_graph: &mut RenderGraph,
        msaa: &Msaa,
    ) -> String {
        let swapchain_label = format!("{}_swapchain", predator_name);
        let camera_label = format!("{}_camera", predator_name);
        let pass_label = format!("{}_pass", predator_name);
        let window_depth_label =
            format!("{}_window_depth_texture", predator_name);

        let window_id = WindowId::new();
        create_window_events.send(CreateWindow {
            id: window_id,
            descriptor: WindowDescriptor {
                title: predator_name.to_string(),
                ..Default::default()
            },
        });

        // TODO: Understand what this does.
        render_graph.add_node(
            swapchain_label.clone(),
            WindowSwapChainNode::new(window_id),
        );
        render_graph.add_node(
            window_depth_label.clone(),
            WindowTextureNode::new(
                window_id,
                TextureDescriptor {
                    format: TextureFormat::Depth32Float,
                    usage: TextureUsage::OUTPUT_ATTACHMENT,
                    sample_count: msaa.samples,
                    ..Default::default()
                },
            ),
        );

        // Registers camera node.
        render_graph.add_system_node(
            camera_label.clone(),
            CameraNode::new(predator_name.clone()),
        );

        active_cameras.add(&predator_name);

        // TODO: Understand what this does.
        let mut window_pass = PassNode::<&MainPass>::new(PassDescriptor {
            color_attachments: vec![msaa.color_attachment_descriptor(
                TextureAttachment::Input("color_attachment".to_string()),
                TextureAttachment::Input("color_resolve_target".to_string()),
                Operations {
                    load: LoadOp::Clear(Color::rgb(0.8, 0.8, 0.8)),
                    store: true,
                },
            )],
            depth_stencil_attachment: Some(
                RenderPassDepthStencilAttachmentDescriptor {
                    attachment: TextureAttachment::Input("depth".to_string()),
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                },
            ),
            sample_count: msaa.samples,
        });
        window_pass.add_camera(&predator_name);
        render_graph.add_node(pass_label.clone(), window_pass);

        // Pipes swapchain into the pass.
        render_graph
            .add_slot_edge(
                swapchain_label.clone(),
                WindowSwapChainNode::OUT_TEXTURE,
                pass_label.clone(),
                if msaa.samples > 1 {
                    "color_resolve_target"
                } else {
                    "color_attachment"
                },
            )
            .expect("Cannot add swapchain slot to window pass");

        // Pipes texture into the pass.
        render_graph
            .add_slot_edge(
                window_depth_label,
                WindowTextureNode::OUT_TEXTURE,
                pass_label.clone(),
                "depth",
            )
            .expect("Cannot add slot edge from depth texture to window pass");

        // Pipes pass into the camera.
        render_graph
            .add_node_edge(camera_label, pass_label)
            .expect("Cannot add camera to window pass");

        predator_name
    }
}
