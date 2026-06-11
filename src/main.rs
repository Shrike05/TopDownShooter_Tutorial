use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet {
    travel_direction: Dir3,
    speed: f32,
}

#[derive(Resource)]
pub struct FireRateTimer {
    timer: Timer,
}

//This is where the program starts
fn main() {
    //Create the app
    let mut app = App::new();

    //Add the default plugin, additionally sets image scaling to default to "nearest neighbour" for pixelart
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    //Insert Resources
    app.insert_resource(FireRateTimer {
        timer: Timer::from_seconds(0.5, TimerMode::Once),
    });

    //Setup the game
    app.add_systems(Startup, (create_camera, create_player));

    //Update system
    app.add_systems(
        Update,
        (move_player, look_at_mouse, spawn_bullet, move_bullet),
    );

    //Run the app
    app.run();
}

//Creates a camera and makes it look at the origin
fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(PerspectiveProjection::default()),
        Transform::from_xyz(0., 0., -50.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

//Loads a player sprite then spawns it at the origin
fn create_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_image_handle: Handle<Image> = asset_server.load("sprites/player.png");
    let mut player_sprite = Sprite::from_image(player_image_handle);
    player_sprite.custom_size = Some(Vec2::new(4., 4.));
    commands.spawn((player_sprite, Transform::from_xyz(0., 0., 0.), Player));
}

//Moves the player
fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = player_query.iter_mut().next().unwrap();
    let speed = 20.;

    if keys.pressed(KeyCode::KeyW) {
        player_transform.translation.y += speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyS) {
        player_transform.translation.y -= speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyD) {
        player_transform.translation.x -= speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyA) {
        player_transform.translation.x += speed * time.delta_secs();
    }
}

//Direct The player towards the mouse
fn look_at_mouse(
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let mut player = player_query.iter_mut().next().unwrap();

    let cursor_pos = match window.cursor_position() {
        Some(position) => position,
        None => return,
    };

    let (camera, camera_transform) = camera_query.iter().next().expect("Cannot find camera");

    // Convert cursor to world ray
    if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
        // Camera position (world coordinates)
        let ray_origin = camera_transform.translation();

        // Ray direction (normalized)
        let ray_dir = ray.direction.normalize();

        // Solve for intersection with XY plane (y = 0)
        if ray_dir.z.abs() > 1e-6 {
            // Avoid division by zero
            let t = -ray_origin.z / ray_dir.z;
            if t >= 0.0 {
                // Valid intersection in front of camera
                let hit_point = ray_origin + t * ray_dir;

                let pos = player.translation;
                let player_forward = player.right().as_vec3();
                let difference_vector = (pos - hit_point).normalize();
                let angle_between = player_forward.dot(difference_vector);

                player.rotate_z(angle_between);
            }
        }
    }
}

//Spawn bullets if the player presses the left mouse button
fn spawn_bullet(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    player_query: Query<&mut Transform, With<Player>>,
    mut firerate_timer: ResMut<FireRateTimer>,
    time: Res<Time>,
) {
    if !firerate_timer.timer.is_finished() {
        firerate_timer.timer.tick(time.delta());
        return;
    }
    firerate_timer.timer.reset();

    if mouse_input.pressed(MouseButton::Left) {
        let player_transform = player_query.iter().next().expect("There is no player");
        //Shoot a bullet
        let bullet_image_handle = asset_server.load("sprites/bullet.png");
        let mut bullet_sprite = Sprite::from_image(bullet_image_handle);
        bullet_sprite.custom_size = Some(Vec2::new(1., 1.));
        let bullet_position = Transform::from_translation(
            player_transform.translation + player_transform.up().as_vec3() * 2.,
        );
        commands.spawn((
            bullet_sprite,
            bullet_position,
            Bullet {
                travel_direction: player_transform.up(),
                speed: 50.,
            },
        ));
    }
}

//Move the bullets forward
fn move_bullet(mut bullets_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut bullet_transform, bullet) in bullets_query.iter_mut() {
        bullet_transform.translation +=
            bullet.travel_direction.as_vec3() * bullet.speed * time.delta_secs();
    }
}
