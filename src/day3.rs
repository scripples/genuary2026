use bevy::color::palettes::css::WHITE;
use bevy::sprite_render::AlphaMode2d;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, window::WindowResolution};
use image::{GenericImageView, ImageReader};

fn main() {
    let img = ImageReader::open("./assets/textures/cat.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let dims = img.dimensions();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(dims.0, dims.1).with_scale_factor_override(1.0),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .run();
}

#[derive(Component)]
struct FibData {
    base: f32,
    hue: f32,
}

fn update(
    mut commands: Commands,
    mut query: Query<(&FibData, &mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (fib, mat) in query.iter_mut() {
        let mat_handle = mat.0.clone();
        let mat = materials.get_mut(&mat_handle).unwrap();
        let hue = ((time.elapsed_secs() * fib.base * 10.) + fib.hue) % 360.;
        mat.color.set_hue(hue);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    //let img = asset_server.load("./textures/cat.jpg");

    commands.spawn((
        Camera2d,
        // Transform ensures the camera space is 0,0 at bottom left corner
        Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
    ));
    // Spawn the image plane
    // commands.spawn((
    //     Mesh2d(meshes.add(Rectangle::new(window.width(), window.height()))),
    //     MeshMaterial2d(materials.add(ColorMaterial {
    //         texture: Some(img),
    //         alpha_mode: AlphaMode2d::Opaque,
    //         ..default()
    //     })),
    // ));
    let rect_height = window.height() / 15.;
    let rect_width = window.width() / 3.;
    let norm_height = rect_height / window.height();
    let rect = Rectangle::new(rect_width, rect_height);

    for i in 0..15 {
        let i_offset = i + 1;
        let fib = fibonacci((i - 7_i32).abs() as u64) as f32;

        // 3 wide
        for j in 0..3 {
            let j_offset = j + 1;

            let hue = (360 / 3 * j_offset) as f32;
            let val = if j_offset % 2 == 0 {
                norm_height * i as f32
            } else {
                1. - norm_height * i as f32
            };

            let color = Color::hsv(hue, 1., val);

            commands.spawn((
                Mesh2d(meshes.add(rect)),
                Transform::from_xyz(
                    rect_width * j_offset as f32 - rect_width / 2.,
                    rect_height * i_offset as f32 - rect_height / 2.,
                    0.,
                ),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color,
                    ..Default::default()
                })),
                FibData { base: fib, hue },
            ));
        }

        // commands.spawn((
        //     Mesh2d(meshes.add(rect)),
        //     Transform::from_xyz(0., rect_height, 0.),
        //     MeshMaterial2d(materials.add(ColorMaterial {
        //         color,
        //         ..Default::default()
        //     })),
        //     FibData { fib, left: true },
        // ));
    }
}

fn fibonacci(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 0;
    let mut count = 0;

    while count < n {
        let tmp = a + b;
        b = a;
        a = tmp;
        count += 1;
    }

    b
}
