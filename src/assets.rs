use crate::constants::*;
use bevy::prelude::*;

pub struct MaterialsAndTextures {
    pub ui_material: Handle<ColorMaterial>,
    pub button_material: Handle<ColorMaterial>,

    pub time_of_day1_material: Handle<ColorMaterial>,
    pub time_of_day2_material: Handle<ColorMaterial>,
    pub time_of_day3_material: Handle<ColorMaterial>,
    pub time_of_day4_material: Handle<ColorMaterial>,

    pub nine_patch_texture: Handle<Texture>,

    pub char1_atlas: Handle<TextureAtlas>,
    pub char2_atlas: Handle<TextureAtlas>,
    pub char3_atlas: Handle<TextureAtlas>,

    pub char1_portrait_material: Handle<ColorMaterial>,
    pub char2_portrait_material: Handle<ColorMaterial>,
    pub char3_portrait_material: Handle<ColorMaterial>,

    pub wolf_atlas: Handle<TextureAtlas>,
    pub bear_atlas: Handle<TextureAtlas>,
    pub troll_atlas: Handle<TextureAtlas>,

    pub canyon_material: Handle<ColorMaterial>,
    pub boulder_material: Handle<ColorMaterial>,
    pub tree_material: Handle<ColorMaterial>,
    pub obelisk_material: Handle<ColorMaterial>,

    pub healthbar_material: Handle<ColorMaterial>,

    pub main_font: Handle<Font>,

    pub splatter_atlas: Handle<TextureAtlas>,

    pub attacking_obelisk_audio: Handle<AudioSource>,
    pub clang_audio: Handle<AudioSource>,
    pub everywhere_audio: Handle<AudioSource>,
    pub here_they_come_audio: Handle<AudioSource>,
    pub leaving_audio: Handle<AudioSource>,
    pub more_of_them_audio: Handle<AudioSource>,
    pub obelisk_fallen_audio: Handle<AudioSource>,
    pub protect_obelisk_audio: Handle<AudioSource>,
    pub whoosh_audio: Handle<AudioSource>,
    pub moving_audio: Handle<AudioSource>,
}

impl FromResources for MaterialsAndTextures {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let mut texture_atlases = resources.get_mut::<Assets<TextureAtlas>>().unwrap();
        let asset_server = resources.get_mut::<AssetServer>().unwrap();

        // load sprite atlases for players
        let handle_player: Handle<Texture> = Handle::from_u128(CHARACTER_1_SPRITE);
        let texture_atlas_player =
            TextureAtlas::from_grid(handle_player, Vec2::new(128., 32.), 4, 1);
        let char1_atlas = texture_atlases.add(texture_atlas_player);

        let handle_player2: Handle<Texture> = Handle::from_u128(CHARACTER_2_SPRITE);
        let texture_atlas_player2 =
            TextureAtlas::from_grid(handle_player2, Vec2::new(128., 32.), 4, 1);
        let char2_atlas = texture_atlases.add(texture_atlas_player2);

        let handle_player3: Handle<Texture> = Handle::from_u128(CHARACTER_3_SPRITE);
        let texture_atlas_player3 =
            TextureAtlas::from_grid(handle_player3, Vec2::new(128., 32.), 4, 1);
        let char3_atlas = texture_atlases.add(texture_atlas_player3);

        let handle_splatter: Handle<Texture> = Handle::from_u128(SPLATTER_ID);
        let atlas_splatter =
            TextureAtlas::from_grid(handle_splatter, Vec2::new(32. * 6., 32.), 4, 1);
        let splatter_atlas = texture_atlases.add(atlas_splatter);

        let handle_wolf: Handle<Texture> = Handle::from_u128(ENEMY_WOLF_SPRITE);
        let atlas_wolf = TextureAtlas::from_grid(handle_wolf, Vec2::new(128., 32.), 4, 1);
        let wolf_atlas = texture_atlases.add(atlas_wolf);

        let handle_bear: Handle<Texture> = Handle::from_u128(ENEMY_BEAR_SPRITE);
        let atlas_bear = TextureAtlas::from_grid(handle_bear, Vec2::new(128., 32.), 4, 1);
        let bear_atlas = texture_atlases.add(atlas_bear);

        let handle_troll: Handle<Texture> = Handle::from_u128(ENEMY_TROLL_SPRITE);
        let atlas_troll = TextureAtlas::from_grid(handle_troll, Vec2::new(128., 32.), 4, 1);
        let troll_atlas = texture_atlases.add(atlas_troll);

        MaterialsAndTextures {
            ui_material: materials.add(Color::NONE.into()),
            button_material: materials.add(Color::rgba_u8(70, 70, 70, 30).into()),
            main_font: asset_server.load("assets/fonts/teletactile.ttf").unwrap(),

            time_of_day1_material: materials.add(Handle::from_u128(TIME_OF_DAY_SPRITE1_ID).into()),
            time_of_day2_material: materials.add(Handle::from_u128(TIME_OF_DAY_SPRITE2_ID).into()),
            time_of_day3_material: materials.add(Handle::from_u128(TIME_OF_DAY_SPRITE3_ID).into()),
            time_of_day4_material: materials.add(Handle::from_u128(TIME_OF_DAY_SPRITE4_ID).into()),

            nine_patch_texture: Handle::from_u128(UI_CONTAINER_ID),

            char1_atlas,
            char2_atlas,
            char3_atlas,

            wolf_atlas,
            bear_atlas,
            troll_atlas,

            char1_portrait_material: materials.add(Handle::from_u128(CHARACTER_1_PORTRAIT).into()),
            char2_portrait_material: materials.add(Handle::from_u128(CHARACTER_2_PORTRAIT).into()),
            char3_portrait_material: materials.add(Handle::from_u128(CHARACTER_3_PORTRAIT).into()),

            canyon_material: materials.add(Handle::from_u128(CANYON_SPRITE_ID).into()),
            boulder_material: materials.add(Handle::from_u128(ROCK_SPRITE_ID).into()),
            tree_material: materials.add(Handle::from_u128(TREE_SPRITE_ID).into()),
            obelisk_material: materials.add(Handle::from_u128(OBELISK_SPRITE_ID).into()),

            healthbar_material: materials.add(Handle::from_u128(HEALTHBAR_SPRITE_ID).into()),

            splatter_atlas,

            attacking_obelisk_audio: asset_server
                .load("assets/audio/attacking_obelisk.mp3")
                .unwrap(),
            clang_audio: asset_server.load("assets/audio/clang.mp3").unwrap(),
            everywhere_audio: asset_server.load("assets/audio/everywhere.mp3").unwrap(),
            here_they_come_audio: asset_server
                .load("assets/audio/here_they_come.mp3")
                .unwrap(),
            leaving_audio: asset_server.load("assets/audio/leaving.mp3").unwrap(),
            more_of_them_audio: asset_server.load("assets/audio/more_of_them.mp3").unwrap(),
            moving_audio: asset_server.load("assets/audio/moving.mp3").unwrap(),
            obelisk_fallen_audio: asset_server
                .load("assets/audio/obelisk_fallen.mp3")
                .unwrap(),
            protect_obelisk_audio: asset_server
                .load("assets/audio/protect_obelisk.mp3")
                .unwrap(),
            whoosh_audio: asset_server.load("assets/audio/whoosh1.mp3").unwrap(),
        }
    }
}
