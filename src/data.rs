use bevy::prelude::*;
use spectre_loaders::data_loaders::DataFileLoader;

use crate::abilities::ability_data::AbilityDatabase;

pub struct DataFileLoaderPlugin;

impl Plugin for DataFileLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // not used as I'm hard coding in abilities
        app.add_asset::<AbilityDatabase>()
            .add_asset_loader_from_instance::<AbilityDatabase, DataFileLoader>(
                DataFileLoader::from_extensions(vec!["abr"]),
            );
    }
}
