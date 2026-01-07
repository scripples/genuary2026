use bevy::color::palettes::css::{BLACK, ORANGE_RED};
use bevy::math::ops::sin;
use bevy::mesh::CircleMeshBuilder;
use bevy::prelude::*;
use bevy::sprite_render::AlphaMode2d;
use bevy::window::PrimaryWindow;
use noiz::cells::Voronoi;
use noiz::prelude::{BlendCellValues, SimplecticBlend};
use noiz::rng::{Random, SNorm};
use noiz::{Noise, SampleableFor, ScalableNoise, SeedableNoise};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // In this example, we will use a fixed timestep to draw a pattern on the screen
        // one pixel at a time, so the pattern will gradually emerge over time, and
        // the speed at which it appears is not tied to the framerate.
        // Let's make the fixed update very fast, so it doesn't take too long. :)
        // .insert_resource(Time::<Fixed>::from_hz(1024.0))
        .insert_resource(VoronoiSpots2D::new(1.0, 1.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .run();
}

#[derive(Component)]
struct CircleMesh {
    center: Vec3,
    base_angle: Quat,
}

#[derive(Resource)]
struct VoronoiSpots2D {
    d1: Noise<BlendCellValues<Voronoi, SimplecticBlend, Random<SNorm, f32>>>,
    d2: Noise<BlendCellValues<Voronoi, SimplecticBlend, Random<SNorm, f32>>>,
}

impl VoronoiSpots2D {
    fn new(noise_scale: f32, noise_period: f32) -> Self {
        let mut noise_a =
            Noise::<BlendCellValues<Voronoi, SimplecticBlend, Random<SNorm, f32>>>::default();
        noise_a.set_seed(420);
        noise_a.set_frequency(noise_scale);
        noise_a.set_period(noise_period);

        let mut noise_b =
            Noise::<BlendCellValues<Voronoi, SimplecticBlend, Random<SNorm, f32>>>::default();
        noise_b.set_seed(69);
        noise_b.set_frequency(noise_scale);
        noise_b.set_period(noise_period);

        Self {
            d1: noise_a,
            d2: noise_b,
        }
    }

    fn sample2d(&self, p1: Vec3, p2: Vec3) -> (f32, f32) {
        (self.d1.sample(p1), self.d2.sample(p2))
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    commands.spawn((
        Camera2d,
        // Transform ensures the camera space is 0,0 at bottom left corner
        Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
    ));

    // To make it extra fancy, we can set the Alpha of each pixel,
    // so that it fades out in a circular fashion.
    let y_dim = window.height() as i32;
    let x_dim = window.width() as i32;

    // Create an iterator from the input coordinates, sample it, then stretch.
    // Collect to a vector out here because keeping as iterator makes the compiler angry in the
    // inner x for loop, so we just take a reference to this later. This only happens once on
    // startup so it's not so bad but I wouldn't want to do this on the update loop.
    let y_spawn_map: Vec<i32> = (0..y_dim).step_by(8).map(|i| i * 8).collect();
    let x_spawn_map: Vec<i32> = (0..x_dim).step_by(8).map(|i| i * 8).collect();

    let mut latch = false;

    for y in y_spawn_map.iter() {
        // Hit the latch every X and Y for alternating in two dimensions
        latch = !latch;
        for x in x_spawn_map.iter() {
            latch = !latch;
            let color = if latch { BLACK } else { ORANGE_RED };
            let circle = CircleMeshBuilder::new(44.0, 32);
            commands.spawn((
                Mesh2d(meshes.add(circle)),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: color.into(),
                    alpha_mode: AlphaMode2d::Opaque,
                    ..default()
                })),
                Transform::from_xyz(*x as f32, *y as f32, 0.),
                CircleMesh {
                    center: Vec3::new(*x as f32, *y as f32, 0.),
                    base_angle: Quat::default(),
                },
            ));
        }
    }
}

fn update(circles: Query<(&mut Transform, &CircleMesh)>, time: Res<Time>) {
    // let noise_scale = 44.0;

    for (mut transform, center) in circles {
        // Apply noise
        // let s1 = Vec3::new(center.center.x, center.center.y, time.elapsed_secs() * 0.5);
        transform.translation.x =
            center.center.x + sin(time.elapsed_secs()) * 20. * sin(transform.translation.y / 64.0);
    }
}
