use bevy::prelude::*;
use bevy::render::{camera, pass, render_graph, render_graph::base};
use bevy::ui;

pub mod node {
    pub const HUD_CAMERA: &str = "hud_camera";
    pub const HUD_PASS: &str = "hud_pass";
    pub const HUD_NODE: &str = "hud_node";
}

#[derive(Debug, Clone, Default, bevy::render::renderer::RenderResources)]
pub struct HUDPass;

#[derive(Bundle, Debug)]
pub struct HUDCameraBundle {
    pub camera: camera::Camera,
    pub perspective_projection: camera::PerspectiveProjection,
    pub visible_entities: camera::VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for HUDCameraBundle {
    fn default() -> Self {
        let PerspectiveCameraBundle {
            camera,
            perspective_projection,
            visible_entities,
            transform,
            global_transform
        } = PerspectiveCameraBundle::with_name(node::HUD_CAMERA);
        Self {
            camera,
            perspective_projection,
            visible_entities,
            transform,
            global_transform
        }
    }
}

fn init_hud(
    msaa: Res<Msaa>,
    mut graph: ResMut<render_graph::RenderGraph>,
    mut active_cams: ResMut<bevy::render::camera::ActiveCameras>,
) {
    // Create pass node
    let mut hud_pass_node = render_graph::PassNode::<&HUDPass>::new(pass::PassDescriptor {
        color_attachments: vec![pass::RenderPassColorAttachmentDescriptor {
            attachment: pass::TextureAttachment::Input("color_attachment".to_string()),
            resolve_target: Some(pass::TextureAttachment::Input("color_resolve_target".to_string())),
            ops: pass::Operations {
                load: pass::LoadOp::Load,
                store: true,
            },
        }],
        depth_stencil_attachment: Some(pass::RenderPassDepthStencilAttachmentDescriptor {
            attachment: pass::TextureAttachment::Input("depth".to_string()),
            depth_ops: Some(pass::Operations {
                load: pass::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        }),
        sample_count: msaa.samples,
    });

    // Add camera to node
    hud_pass_node.add_camera(node::HUD_CAMERA);
    graph.add_node(node::HUD_PASS, hud_pass_node);

    // Connect main data to hud pass
    graph.add_slot_edge(
        base::node::PRIMARY_SWAP_CHAIN,
        render_graph::WindowSwapChainNode::OUT_TEXTURE,
        node::HUD_PASS,
        if msaa.samples > 1 {
            "color_resolve_target"
        } else {
            "color_attachment"
        },
    ).unwrap();

    graph.add_slot_edge(
        base::node::MAIN_DEPTH_TEXTURE,
        render_graph::WindowTextureNode::OUT_TEXTURE,
        node::HUD_PASS,
        "depth",
    ).unwrap();

    if msaa.samples > 1 {
        graph.add_slot_edge(
            base::node::MAIN_SAMPLED_COLOR_ATTACHMENT,
            render_graph::WindowSwapChainNode::OUT_TEXTURE,
            node::HUD_PASS,
            "color_attachment",
        ).unwrap();
    }

    // Add camera to graph and connect camera to hud pass
    graph.add_system_node(node::HUD_CAMERA, render_graph::CameraNode::new(node::HUD_CAMERA));
    graph.add_node_edge(node::HUD_CAMERA, node::HUD_PASS).unwrap();

    // Connect main pass to hud pass to ui pass
    graph.add_node_edge(base::node::MAIN_PASS, node::HUD_PASS).unwrap();
    graph.add_node_edge(node::HUD_PASS, ui::node::UI_PASS).unwrap();

    // Add hud resource and connect to hud pass
    graph.add_system_node(node::HUD_NODE, render_graph::RenderResourcesNode::<HUDPass>::new(true));
    graph.add_node_edge(node::HUD_NODE, node::HUD_PASS).unwrap();

    // Activate camera
    active_cams.add(node::HUD_CAMERA);
}

fn add_hud(
    mut commands: Commands,

    assets: Res<AssetServer>,
    textures: Res<Assets<Texture>>,
    mut smaterials: ResMut<Assets<StandardMaterial>>,
    mut cmaterials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn().insert_bundle(HUDCameraBundle::default());
    // commands.spawn().insert_bundle(PerspectiveCameraBundle::with_name(node::HUD_CAMERA));

    commands.spawn().insert_bundle(PbrBundle {
        mesh: assets.get_handle(format!("models/maps/monke.glb#Mesh0/Primitive0").as_str()),
        material: smaterials.add(Color::rgb(0.6, 0.9, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(2.0, -1.0, -5.0)),
        ..Default::default()
    }).remove::<base::MainPass>().insert(HUDPass);

    commands.spawn().insert_bundle(UiCameraBundle::default());

    commands.spawn().insert_bundle(TextBundle {
        text: Text::with_section(
            "Text",
            TextStyle {
                font: assets.load("JosefinSans-Regular.ttf"),
                font_size: 90.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
            },
        ),
        ..Default::default()
    });

    let crosshair = textures.get("crosshair.png").unwrap();

    commands.spawn().insert_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: ui::JustifyContent::Center,
            align_items: ui::AlignItems::FlexEnd,
            ..Default::default()
        },
        material: cmaterials.add(Color::NONE.into()),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn().insert_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(crosshair.size.width as f32), Val::Px(crosshair.size.height as f32)),
                align_self: ui::AlignSelf::Center,
                ..Default::default()
            },
            material: cmaterials.add(assets.load("crosshair.png").into()),
            ..Default::default()
        });
    });
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_hud.system());
        app.add_system_set(SystemSet::on_enter(crate::AppState::Loaded).with_system(add_hud.system()));
    }
}
