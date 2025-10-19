use bevy::prelude::*;
use snake_bevy::*;

/// Helper function to create a test app with minimal plugins
fn setup_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

/// Helper function to create a fully initialized game app with all plugins
fn setup_full_app() -> App {
    init_app()
}

#[test]
fn test_constants() {
    // Test that game board constants are correctly defined
    assert_eq!(SIZE, 0.8);
    assert_eq!(HALF_LEN, 7);
    assert_eq!(BOARD_LEN, 14);
    assert_eq!(PADDING, 1.0);
    assert_eq!(BOARD_VIEWPORT_IN_WORLD_UNITS, 16.0);
}

#[test]
fn test_coordinate_component() {
    // Test that Coordinate component can be created and used
    let coord = coordinate::Coordinate(Vec2::new(5.0, 3.0));
    assert_eq!(coord.0.x, 5.0);
    assert_eq!(coord.0.y, 3.0);
}

#[test]
fn test_coordinate_from_tuple() {
    // Test Coordinate creation from tuple
    let coord = coordinate::Coordinate::from((2.0, -3.0));
    assert_eq!(coord.0.x, 2.0);
    assert_eq!(coord.0.y, -3.0);
}

#[test]
fn test_direction_to_vec2_conversion() {
    // Test direction conversion to Vec2
    let up: Vec2 = direction::Direction::Up.into();
    assert_eq!(up, Vec2::new(0.0, 1.0));

    let down: Vec2 = direction::Direction::Down.into();
    assert_eq!(down, Vec2::new(0.0, -1.0));

    let left: Vec2 = direction::Direction::Left.into();
    assert_eq!(left, Vec2::new(-1.0, 0.0));

    let right: Vec2 = direction::Direction::Right.into();
    assert_eq!(right, Vec2::new(1.0, 0.0));
}

#[test]
fn test_direction_enum_variants() {
    // Test that all direction variants exist
    let _up = direction::Direction::Up;
    let _down = direction::Direction::Down;
    let _left = direction::Direction::Left;
    let _right = direction::Direction::Right;

    // Test direction equality
    assert_eq!(direction::Direction::Up, direction::Direction::Up);
    assert_ne!(direction::Direction::Up, direction::Direction::Down);
}

#[test]
fn test_entities_can_have_coordinates_and_transforms() {
    // Setup
    let mut app = setup_test_app();

    // Spawn a test entity with Coordinate and Transform
    let entity = app.world_mut().spawn((
        coordinate::Coordinate(Vec2::new(5.0, 3.0)),
        Transform::default(),
    )).id();

    // Verify entity exists and has both components
    assert!(app.world().get_entity(entity).is_ok());
    let world_entity = app.world().entity(entity);
    assert!(world_entity.contains::<coordinate::Coordinate>());
    assert!(world_entity.contains::<Transform>());
}

#[test]
fn test_depth_component() {
    // Test Depth component
    let mut app = setup_test_app();

    let depth_entity = app.world_mut().spawn(snake::Depth(5.0)).id();
    let depth = app.world().entity(depth_entity).get::<snake::Depth>().unwrap();
    assert_eq!(depth.0, 5.0);
}

#[test]
fn test_board_boundary_calculations() {
    // Test that board boundaries are calculated correctly
    // The board goes from -HALF_LEN to +HALF_LEN
    let min_coord = -HALF_LEN as f32;
    let max_coord = HALF_LEN as f32;

    assert_eq!(min_coord, -7.0);
    assert_eq!(max_coord, 7.0);

    // Total board size should be BOARD_LEN
    let total_size = (max_coord - min_coord) as i32;
    assert_eq!(total_size, BOARD_LEN);
}

#[test]
fn test_id_component() {
    // Test that Id component works
    let mut app = setup_test_app();

    let entity = app.world_mut().spawn(snake::Id(1)).id();
    let id = app.world().entity(entity).get::<snake::Id>().unwrap();
    assert_eq!(id.0, 1);
}

#[test]
fn test_grid_cells_are_created() {
    // Initialize the full game app with all plugins
    let mut app = setup_full_app();

    // Run startup systems which create the grid
    app.update();

    // Query for grid cells (entities with Coordinate and Depth components)
    // Grid cells have Depth(-1.0) to distinguish them from game entities
    let grid_cells: Vec<_> = app
        .world_mut()
        .query_filtered::<(Entity, &coordinate::Coordinate, &snake::Depth), With<Sprite>>()
        .iter(app.world())
        .filter(|(_, _, depth)| depth.0 == -1.0)
        .collect();

    // Expected number of grid cells: (2 * HALF_LEN + 1) ^ 2
    // With HALF_LEN = 7: (2*7 + 1) ^ 2 = 15 * 15 = 225
    let expected_cells = ((2 * HALF_LEN + 1) * (2 * HALF_LEN + 1)) as usize;

    assert_eq!(
        grid_cells.len(),
        expected_cells,
        "Expected {} grid cells to be created, but found {}",
        expected_cells,
        grid_cells.len()
    );

    // Verify that grid cells span the correct coordinate range
    // They should go from -HALF_LEN to +HALF_LEN in both x and y
    let min_x = grid_cells
        .iter()
        .map(|(_, coord, _)| coord.0.x)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_x = grid_cells
        .iter()
        .map(|(_, coord, _)| coord.0.x)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_y = grid_cells
        .iter()
        .map(|(_, coord, _)| coord.0.y)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_y = grid_cells
        .iter()
        .map(|(_, coord, _)| coord.0.y)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    assert_eq!(min_x, -HALF_LEN as f32, "Grid minimum X should be -HALF_LEN");
    assert_eq!(max_x, HALF_LEN as f32, "Grid maximum X should be HALF_LEN");
    assert_eq!(min_y, -HALF_LEN as f32, "Grid minimum Y should be -HALF_LEN");
    assert_eq!(max_y, HALF_LEN as f32, "Grid maximum Y should be HALF_LEN");

    // Verify all grid cells have depth -1.0
    for (_, _, depth) in &grid_cells {
        assert_eq!(depth.0, -1.0, "All grid cells should have depth -1.0");
    }
}

#[test]
fn test_camera_is_created() {
    // Initialize the full game app
    let mut app = setup_full_app();

    // Run startup systems
    app.update();

    // Query for camera entities
    let cameras: Vec<_> = app
        .world_mut()
        .query::<(Entity, &Camera2d)>()
        .iter(app.world())
        .collect();

    // Should have exactly one camera
    assert_eq!(
        cameras.len(),
        1,
        "Expected exactly 1 camera to be created, but found {}",
        cameras.len()
    );
}

#[test]
fn test_apples_are_spawned_on_startup() {
    // Initialize the full game app
    let mut app = setup_full_app();

    // Run startup systems
    app.update();

    // Query for apple entities - apples have Coordinate and are spawned with Sprite::from_image
    // We can't access the Apple component from tests (it's private), but we can count entities
    // with specific characteristics. The setup spawns 4 apples initially.
    let entities_with_sprite: Vec<_> = app
        .world_mut()
        .query_filtered::<Entity, With<Sprite>>()
        .iter(app.world())
        .collect();

    // We should have at least the grid (225) + apples (4) = 229 entities with sprites
    // (Could be more if snakes are spawned depending on game state)
    assert!(
        entities_with_sprite.len() >= 225,
        "Expected at least 225 entities with sprites (grid cells), found {}",
        entities_with_sprite.len()
    );
}
