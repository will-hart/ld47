/// The nine-patch UI sprite
pub const UI_CONTAINER_ID: u128 = 324890576394765893475;

/// The nine-patch health bar sprite
pub const HEALTHBAR_SPRITE_ID: u128 = 43562893752235235;

/// character 1 spritesheet
pub const CHARACTER_1_SPRITE: u128 = 2340965832048509025;
/// character 2 spritesheet
pub const CHARACTER_2_SPRITE: u128 = 135120965832048509025;
/// character 3 spritesheet
pub const CHARACTER_3_SPRITE: u128 = 768909245132048509025;
/// character 1 portrait
pub const CHARACTER_1_PORTRAIT: u128 = 24356234562462346236;
/// character 2 portrait
pub const CHARACTER_2_PORTRAIT: u128 = 13523462124;
/// character 3 portrait
pub const CHARACTER_3_PORTRAIT: u128 = 34334324526213383;

/// enemy spritesheets
pub const ENEMY_WOLF_SPRITE: u128 = 3456909345645132345625;
pub const ENEMY_BEAR_SPRITE: u128 = 32406238029835205826734;
pub const ENEMY_TROLL_SPRITE: u128 = 12637462356347113252;

/// The margin for 9-patch UI assets (16x16)
pub const UI_SPRITE_MARGIN: f32 = 7.;

pub const RESOLUTION_X: u32 = 1280;
pub const RESOLUTION_Y: u32 = 720;

pub const GAME_ELEMENT_LAYER: f32 = 3.;

// location of top left corner in UI coords
// pub const GAME_OFFSET_X: f32 = -640.; // negative half RESOLUTION_X
// pub const GAME_OFFSET_Y: f32 = 360.; // half RESOLUTION_Y

// PROBABLY SHOULDN'T BE HARDCODED, BUT GAME JAM
pub const SPAWN_LOCATIONS: [(f32, f32); 3] = [(-480., 365.), (-180., 365.), (120., 365.)];
pub const TARGET_LOCATIONS: [(f32, f32); 3] = [(-480., -300.), (-180., -300.), (120., -300.)];

/// how far the players set up from the target location
pub const PLAYER_OFFSET_Y: f32 = 40.;
pub const PLAYER_OFFSET_X: f32 = 40.;

/// the range that is considered "melee"
pub const MELEE_RANGE: f32 = PLAYER_OFFSET_Y + 2.;

/// PROBABLY SHOULDN'T BE HARDCODED, BUT GAME JAM
pub const ENEMY_SPEED: f32 = 60.;

pub const MIN_LANE: usize = 0;
pub const MAX_LANE: usize = 2;
