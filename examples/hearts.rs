use bevy::{
    prelude::{
        default, App, Camera2dBundle, ClearColor, Color, Commands, Quat, Startup, Transform, Vec2,
    },
    DefaultPlugins,
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, Path, PathBuilder, ShapeBundle, ShapePlugin, FillOptions},
    shapes::Rectangle,
};
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin, RigidBody},
    render::RapierDebugRenderPlugin,
};
use lyon_path_rapier2d_collider::{convex_hull, fill_trimesh};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            ShapePlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let floor_shape = Rectangle {
        extents: Vec2::new(1000.0, 50.0),
        ..default()
    };
    let floor_path = GeometryBuilder::build_as(&floor_shape);
    let floor_collider = convex_hull(&floor_path.0, &FillOptions::DEFAULT).unwrap();

    commands.spawn((
        ShapeBundle {
            path: floor_path,
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        },
        floor_collider,
        RigidBody::Fixed,
        Fill::color(Color::RED),
    ));

    let mut path_builder = PathBuilder::new();

    path_builder.move_to(Vec2::new(0., 0.));
    path_builder.cubic_bezier_to(
        Vec2::new(70., 70.),
        Vec2::new(175., -35.),
        Vec2::new(0., -140.),
    );
    path_builder.cubic_bezier_to(
        Vec2::new(-175., -35.),
        Vec2::new(-70., 70.),
        Vec2::new(0., 0.),
    );
    path_builder.close();

    let path = path_builder.build().0;

    for i in 0..3 {
        let collider = fill_trimesh(&path, &FillOptions::DEFAULT);

        commands.spawn((
            ShapeBundle {
                path: Path(path.clone()),
                transform: Transform::from_xyz(i as f32 * 190.0 - 250.0, i as f32 * 40.0, 1.0)
                    .with_rotation(Quat::from_rotation_z(2.2689)),
                ..default()
            },
            RigidBody::Dynamic,
            Fill::color(Color::BLUE),
            collider,
        ));
    }
}
