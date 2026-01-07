use std::f32::consts::PI;

use bevy::color::palettes::css::WHITE;
use bevy::core_pipeline::core_2d::graph::Node2d;
use bevy::mesh::{CircleMeshBuilder, MeshVertexAttribute, VertexAttributeValues};
use bevy::picking::window;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_math::ops::{asin, sin, tan};
use bevy_math::prelude::*;
use noiz::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .run();
}

#[derive(Component)]
struct Locator;

#[derive(Component)]
struct SinLine {
    tail: bool,
    end: Vec2,
}

#[derive(Component)]
struct LocCircle;

#[derive(Component)]
struct GuideCircle;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let noise =
        Noise::<BlendCellGradients<SimplexGrid, SimplecticBlend, QuickGradients>>::default();

    commands.spawn((
        Camera2d,
        // Transform ensures the camera space is 0,0 at bottom left corner
        Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
    ));

    let window_mid_x = window.width() / 2.;
    let window_mid_y = window.height() / 2.;

    // Draw circle to the right

    let circle_rad = 128.0;
    let circle_pad = 16.0;
    let circle_x = window.width() - circle_pad - circle_rad;

    let circle = CircleMeshBuilder::new(circle_rad, 64).build();
    let circle_polyline = circle.to_polyline_2d();

    let circle_loc = CircleMeshBuilder::new(8.0, 32).build();

    commands
        .spawn((
            Mesh2d(meshes.add(circle_polyline)),
            Transform::from_xyz(circle_x, window_mid_y, 0.),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: WHITE.into(),
                ..Default::default()
            })),
            GuideCircle,
        ))
        .with_children(|p| {
            p.spawn((
                Mesh2d(meshes.add(circle_loc)),
                Transform::from_xyz(0., circle_rad, 0.),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: WHITE.into(),
                    ..Default::default()
                })),
                LocCircle,
            ));
        });

    // Define object spawn space, starting at pad and ending at pad
    // This could be a flexbox but whatev
    let pad = 16.0;
    let spawn_stop = window.width() - pad;
    let mut spawn_pos = pad;

    let rect_width = 2.0;
    let rect_height = 128_f32;
    let num_instances = 128;
    let spacing = (spawn_stop - spawn_pos) / num_instances as f32;

    // let mut vertices = Vec::new();

    // while spawn_pos < spawn_stop {
    //     // let noise_scale: f32 = noise.sample(Vec2::new(spawn_pos * 0.005, 2.0));
    //     let sin_pos = sin(spawn_pos / 100.) * 128.0;
    //     vertices.push(Vec2::new(spawn_pos, sin_pos));

    //     // commands
    //     //     .spawn((
    //     //         Locator,
    //     //         Transform::from_xyz(spawn_pos, window.height() / 2., 0.),
    //     //     ))
    //     //     .with_children(|p| {
    //     //         p.spawn((
    //     //             Mesh2d(meshes.add(Rectangle::new(rect_width, rect_height))),
    //     //             Transform::from_xyz(0., height_offset, 0.)
    //     //                 .with_scale(Vec3::new(1., sin_scale, 1.)),
    //     //             MeshMaterial2d(materials.add(ColorMaterial {
    //     //                 color: WHITE.into(),
    //     //                 ..Default::default()
    //     //             })),
    //     //         ));
    //     //     });
    //     spawn_pos += spacing;
    // }

    // Draw a sine line in the update loop originating from the circle, but I initialize here with
    // a single vertex
    let sin_line = Polyline2d::new(vec![Vec2::new(
        circle_x - circle_rad - circle_pad,
        window_mid_y,
    )]);

    commands.spawn((
        Mesh2d(meshes.add(sin_line)),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: WHITE.into(),
            ..Default::default()
        })),
        SinLine {
            tail: true,
            end: Vec2::new(circle_x - circle_rad - circle_pad, window_mid_y),
        },
    ));
}

pub trait ToPolyline2d {
    fn to_polyline_2d(&self) -> Polyline2d;
}

/// Assumes X-Z plane object. Duplicates the first entry as the last for a connected mesh.
impl ToPolyline2d for Mesh {
    fn to_polyline_2d(&self) -> Polyline2d {
        let v = self.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        let v: Vec<[f32; 3]> = v.as_float3().unwrap().to_vec();
        let mut v: Vec<Vec2> = v.iter().map(|v| Vec2::new(v[0], v[1])).collect();
        v.push(v.first().unwrap().clone());

        Polyline2d::new(v)
    }
}

fn update(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(&mut Mesh2d, &mut Transform, &mut SinLine), With<SinLine>>,
        Single<&mut Transform, With<GuideCircle>>,
        Single<&mut GlobalTransform, With<LocCircle>>,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let elapsed = time.elapsed_secs();
    let elapsed_last = time.delta_secs();
    // transform existing points
    let mut last_line = Vec2::default();
    for mut seg in set.p0() {
        seg.1.translation.x -= elapsed_last * 100.;
        if seg.2.tail {
            seg.2.tail = false;
            last_line = seg.2.end;
        }
    }

    // transform guide circle, one degree per second (takes radians as an arg)
    set.p1().rotate_z(sin(elapsed).abs() * 0.1);

    let p2_t = set.p2().translation();
    let new_vert = Vec2::new(last_line.x, p2_t.y);

    let pline = Polyline2d::new(vec![last_line, new_vert]);
    commands.spawn((
        Mesh2d(meshes.add(pline)),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: WHITE.into(),
            ..Default::default()
        })),
        SinLine {
            tail: true,
            end: new_vert,
        },
    ));
}

// Lousy attempt to modify a polyline2d during draw. Really each line should proabably just be its
// own entity if they don't squash or stretch. Either way this shouldTM be easier.
// fn update(
//     mut set: ParamSet<(
//         Single<(&mut Mesh2d, &mut Transform), With<SinLine>>,
//         Single<&mut Transform, With<GuideCircle>>,
//         Single<&mut GlobalTransform, With<LocCircle>>,
//     )>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     time: Res<Time>,
// ) {
//     let elapsed = time.elapsed_secs();
//     let elapsed_last = time.delta_secs();
//     // transform existing points
//     set.p0().1.translation.x -= elapsed;
//
//     // transform guide circle, one degree per second (takes radians as an arg)
//     set.p1().rotate_z(sin(elapsed).abs() * 0.1);
//
//     // get location of loc circle, use it to spawn a point for the readout
//     let loc_pos = set.p2().translation();
//     println!("{}", loc_pos);
//     let sin_line_mesh = meshes.get_mut(&set.p0().0.0).unwrap();
//
//     let mut vertices = sin_line_mesh
//         .attribute_mut(Mesh::ATTRIBUTE_POSITION)
//         .unwrap()
//         .as_float3()
//         .unwrap()
//         .to_vec();
//
//     let indices = sin_line_mesh.indices_mut().unwrap();
//
//     vertices.push(loc_pos.to_array());
//     indices.push(indices.len() as u32);
//     meshes.set_changed();
// }
