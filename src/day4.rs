use bevy::camera::RenderTarget;
use bevy::color::palettes::css::WHITE;
use bevy::sprite_render::AlphaMode2d;
use bevy::window::{PrimaryWindow, WindowRef};
use bevy::{prelude::*, window::WindowResolution};
use image::{GenericImageView, ImageReader};

fn main() {
    let img = ImageReader::open("./assets/textures/cat.jpg")
        .unwrap()
        .decode()
        .unwrap();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let img = asset_server.load("./textures/cat.jpg");
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(64), 16, 16, Some(UVec2::splat(16)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // (
    //     "With Slicing",
    //     style.clone(),
    //     Vec2::new(100.0, 200.0),
    //     SpriteImageMode::Sliced(TextureSlicer {
    //         border: BorderRect::all(slice_border),
    //         center_scale_mode: SliceScaleMode::Stretch,
    //         ..default()
    //     }),
    // ),

    commands.spawn((
        Camera2d,
        // Transform ensures the camera space is 0,0 at bottom left corner
        Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
    ));

    for i in 0..16 {
        for j in 0..16 {
            commands.spawn((
                Sprite {
                    image: img.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: (i * j) + j,
                    }),
                    ..default()
                },
                Transform::from_translation(Vec3::new(i as f32 * 64., j as f32 * 64., 0.0)),
            ));
        }
    }

    // Spawn a second window
    // let second_window = commands
    //     .spawn(Window {
    //         title: "Second window".to_owned(),
    //         ..default()
    //     })
    //     .id();

    // let second_window_camera = commands
    //     .spawn((
    //         Camera2d::default(),
    //         Transform::from_xyz(6.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    //         Camera {
    //             target: RenderTarget::Window(WindowRef::Entity(second_window)),
    //             ..Default::default()
    //         },
    //     ))
    //     .id();
}

fn update(
    mut commands: Commands,
    mut query: Query<(&mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
}
