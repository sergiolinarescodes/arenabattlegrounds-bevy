use bevy::prelude::*;

use crate::handle_object::ObjectBundle;

#[derive(Component)]
pub struct Player {
    pub id: i32,
    pub name: String,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("Default Player"),
        }
    }
}

#[derive(Component)]
pub struct CubeObject {
    body: Body,
    left_hand: ExtremePart,
    right_hand: ExtremePart,
}

#[derive(Component)]
pub struct Item {
    pub item_name: String,
    pub description: String,
    pub id: u32,
    pub price: u32,
    pub rarity: u32,
    pub hp: u32,
    pub action_list: Vec<Action>,
    pub item_sub_type: ItemSubType,
}

pub enum Action {
    // Define your actions here...
}

pub enum ItemSubType {
    Prototype,
    Cyborg,
    Scrap,
    // Add other subtypes here...
}

#[derive(Component)]
struct Body(Entity);

#[derive(Component)]
struct ExtremePart(Entity);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        //app.add_systems(Update, print_items_inside_player);
        app.add_systems(Update, spawn_cube);
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body = commands
        .spawn((Item {
            item_name: String::from("Standard Body Item"),
            description: String::from("This is a standard body item."),
            id: 1,
            price: 100,
            rarity: 1,
            hp: 100,
            action_list: vec![],
            item_sub_type: ItemSubType::Prototype,
        },))
        .id();

    let left_hand = commands
        .spawn((Item {
            item_name: String::from("Standard Left Hand Item"),
            description: String::from("This is a standard left hand item."),
            id: 2,
            price: 50,
            rarity: 1,
            hp: 50,
            action_list: vec![],
            item_sub_type: ItemSubType::Prototype,
        },))
        .id();

    let right_hand = commands
        .spawn((Item {
            item_name: String::from("Standard Right Hand Item"),
            description: String::from("This is a standard right hand item."),
            id: 3,
            price: 50,
            rarity: 1,
            hp: 50,
            action_list: vec![],
            item_sub_type: ItemSubType::Prototype,
        },))
        .id();

    for i in 0..5 {
        commands.spawn((
            Player {
                id: 1,
                name: String::from("Player 1"),
            },
            CubeObject {
                body: Body(body),
                left_hand: ExtremePart(left_hand),
                right_hand: ExtremePart(right_hand),
            },
            ObjectBundle {
                model: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                    material: materials.add(Color::rgb_u8(124, 144, 255).into()),
                    transform: Transform::from_xyz(2.0, 0.2 * i as f32, 0.0),
                    ..default()
                },
            },
        ));
    }

    for i in 0..5 {
        commands.spawn((
            Player {
                id: 1,
                name: String::from("Player 2"),
            },
            CubeObject {
                body: Body(body),
                left_hand: ExtremePart(left_hand),
                right_hand: ExtremePart(right_hand),
            },
            ObjectBundle {
                model: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                    material: materials.add(Color::rgb_u8(222, 10, 255).into()),
                    transform: Transform::from_xyz(-2.0, 0.2 * i as f32, 0.0),
                    ..default()
                },
            },
        ));
    }
}

pub fn print_items_inside_player(player_query: Query<&Player>, item_query: Query<&Item>) {
    for player in player_query.iter() {
        println!("Player: {}", player.name);
        for item in item_query.iter() {
            println!("Item: {}", item.item_name);
        }
    }
}

pub fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(&Player, &mut CubeObject)>,
) {
    for (player) in query.iter_mut() {
        println!("Spawn: {}", player.0.name);
        if mouse_button_input.just_pressed(MouseButton::Left) {
            let translation = Vec3::new(0.0, 0.2 * 6 as f32, 0.0);
            commands.spawn((ObjectBundle {
                model: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                    material: materials.add(Color::rgb_u8(124, 144, 255).into()),
                    transform: Transform::from_translation(translation),
                    ..Default::default()
                },
            },));
        }
    }
}
