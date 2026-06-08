use bevy::prelude::*;

//This is where the program starts
fn main() {
    //Create the app
    let mut app = App::new();

    //Add the default plugin, additionally sets image scaling to default to "nearest neighbour" for pixelart
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    //Setup the game
    app.add_systems(Startup, (create_camera, create_player));

    //Update system
    app.add_systems(Update, move_player);

    //Run the app
    app.run();
}

//Creates a camera, positions it at 0,0,-20 and makes it look at the origin
fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(PerspectiveProjection::default()),
        Transform::from_xyz(0., 0., -20.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

//Loads a player sprite then spawns it at the origin
fn create_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_image_handle: Handle<Image> = asset_server.load("sprites/player.png");
    let mut player_sprite = Sprite::from_image(player_image_handle);
    player_sprite.custom_size = Some(Vec2::new(4., 4.));
    commands.spawn((player_sprite, Transform::from_xyz(0., 0., 0.)));
}

//Moves the player
fn move_player(
    mut player_query: Query<&mut Transform, With<Sprite>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut player_transform = player_query.iter_mut().next().unwrap();
    let speed = 1.;

    if keys.pressed(KeyCode::KeyW) {
        player_transform.translation.y += speed;
    }
    if keys.pressed(KeyCode::KeyS) {
        player_transform.translation.y -= speed;
    }
    if keys.pressed(KeyCode::KeyD) {
        player_transform.translation.x -= speed;
    }
    if keys.pressed(KeyCode::KeyA) {
        player_transform.translation.x += speed;
    }
}
