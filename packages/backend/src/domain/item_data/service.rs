use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use log::{info, warn};

use crate::errors::{AppError, AppResult};

use super::models::{
    AttributeRequirements, BreachstoneInfo, CurrencyData, DefenceValues, EssenceInfo,
    EssenceModifier, FlaskData, GameDataVersion, GemData, ImportedCategory, ImportedItem,
    ImportedVersion, Item, ItemCategory, ItemSearchParams, ItemSearchResult, ModDisplay, OmenInfo,
    ShieldValues, SoulCoreInfo, WeaponValues,
};
use super::traits::{ItemDataRepository, ItemDataService};

pub struct ItemDataServiceImpl {
    repository: Arc<dyn ItemDataRepository>,
    /// Absolute path to the `data/game_data/` directory inside the Tauri resource dir.
    data_dir: PathBuf,
}

impl ItemDataServiceImpl {
    pub fn new(repository: Arc<dyn ItemDataRepository>, data_dir: PathBuf) -> Self {
        Self {
            repository,
            data_dir,
        }
    }

    fn version_path(&self) -> PathBuf {
        self.data_dir.join("version.json")
    }

    fn items_path(&self) -> PathBuf {
        self.data_dir.join("items.json")
    }

    fn categories_path(&self) -> PathBuf {
        self.data_dir.join("item_categories.json")
    }

    fn read_bundled_version(&self) -> Option<ImportedVersion> {
        let path = self.version_path();
        match std::fs::read_to_string(&path) {
            Ok(text) => serde_json::from_str(&text).ok(),
            Err(e) => {
                warn!("Could not read game_data version.json at {path:?}: {e}");
                None
            }
        }
    }

    fn import_from_files(&self) -> AppResult<(Vec<ItemCategory>, Vec<Item>)> {
        // Parse categories
        let cats_text = std::fs::read_to_string(self.categories_path()).map_err(|e| {
            AppError::internal_error(
                "import_from_files",
                &format!("Failed to read item_categories.json: {e}"),
            )
        })?;
        let imported_cats: Vec<ImportedCategory> =
            serde_json::from_str(&cats_text).map_err(|e| {
                AppError::internal_error(
                    "import_from_files",
                    &format!("Failed to parse item_categories.json: {e}"),
                )
            })?;

        let categories: Vec<ItemCategory> = imported_cats
            .into_iter()
            .map(|c| ItemCategory {
                id: c.id,
                name: c.name,
            })
            .collect();

        // Parse items
        let items_text = std::fs::read_to_string(self.items_path()).map_err(|e| {
            AppError::internal_error(
                "import_from_files",
                &format!("Failed to read items.json: {e}"),
            )
        })?;
        let imported_items: Vec<ImportedItem> = serde_json::from_str(&items_text).map_err(|e| {
            AppError::internal_error(
                "import_from_files",
                &format!("Failed to parse items.json: {e}"),
            )
        })?;

        let items: Vec<Item> = imported_items
            .into_iter()
            .map(convert_imported_item)
            .collect();

        Ok((categories, items))
    }
}

#[async_trait]
impl ItemDataService for ItemDataServiceImpl {
    async fn ensure_data_imported(&self) -> AppResult<()> {
        // Check what's bundled
        let bundled_version = match self.read_bundled_version() {
            Some(v) => v,
            None => {
                warn!("No bundled game data found at {:?}. Run `pnpm extract:gamedata` to generate it.", self.data_dir);
                return Ok(());
            }
        };

        // Check what's in the DB
        let db_version = self.repository.get_version().await?;

        let needs_import = match &db_version {
            None => {
                info!("No game data in DB; importing from bundled files.");
                true
            }
            Some(v) if v.patch_version != bundled_version.patch_version => {
                info!(
                    "Game data version changed: DB={}, bundled={}. Re-importing.",
                    v.patch_version, bundled_version.patch_version
                );
                true
            }
            // Same patch_version but a newer extraction: lets us ship in-place
            // fixes (e.g. recategorisation) without bumping the game patch.
            Some(v) if v.extracted_at != bundled_version.extracted_at => {
                info!(
                    "Game data re-extracted: DB={}, bundled={}. Re-importing.",
                    v.extracted_at, bundled_version.extracted_at
                );
                true
            }
            Some(v) => {
                info!(
                    "Game data is current (patch {}). No import needed.",
                    v.patch_version
                );
                false
            }
        };

        if !needs_import {
            return Ok(());
        }

        let patch_version = &bundled_version.patch_version;
        let extracted_at = &bundled_version.extracted_at;

        info!("Importing game data for patch {patch_version}…");

        let (categories, items) = self.import_from_files()?;

        info!(
            "Loaded {} categories and {} items from bundled JSON",
            categories.len(),
            items.len()
        );

        self.repository
            .import_data(patch_version, extracted_at, &categories, &items)
            .await?;

        info!("Game data import complete for patch {patch_version}.");
        Ok(())
    }

    async fn get_item(&self, id: &str) -> AppResult<Option<Item>> {
        self.repository.get_item(id).await
    }

    async fn search_items(&self, params: ItemSearchParams) -> AppResult<ItemSearchResult> {
        self.repository.search_items(&params).await
    }

    async fn get_categories(&self) -> AppResult<Vec<ItemCategory>> {
        self.repository.get_categories().await
    }

    async fn get_version(&self) -> AppResult<Option<GameDataVersion>> {
        self.repository.get_version().await
    }

    async fn toggle_favorite(&self, item_id: &str) -> AppResult<bool> {
        self.repository.toggle_favorite(item_id).await
    }

    async fn get_favorites(&self) -> AppResult<Vec<Item>> {
        self.repository.get_favorites().await
    }

    async fn get_item_by_name(&self, name: &str) -> AppResult<Option<Item>> {
        self.repository.get_item_by_name(name).await
    }
}

// ---------------------------------------------------------------------------
// Conversion: ImportedItem → Item
// ---------------------------------------------------------------------------

fn convert_imported_item(imp: ImportedItem) -> Item {
    let requirements = imp
        .requirements
        .as_ref()
        .map_or_else(AttributeRequirements::default, |r| AttributeRequirements {
            str_req: r.str_val(),
            dex_req: r.dex_val(),
            int_req: r.int_val(),
        });

    let defences = imp.defences.map(|d| DefenceValues {
        armour: d.armour,
        evasion: d.evasion,
        energy_shield: d.energy_shield,
        ward: d.ward,
        movement_speed: d.movement_speed,
    });

    let weapon = imp.weapon.map(|w| WeaponValues {
        damage_min: w.damage_min,
        damage_max: w.damage_max,
        critical: w.critical,
        attack_speed: w.attack_speed,
        range_max: w.range_max,
        reload_time: w.reload_time,
    });

    let shield = imp.shield.map(|s| ShieldValues { block: s.block });

    let gem = imp.gem.map(|g| GemData {
        gem_type: g.gem_type,
        gem_colour: g.gem_colour,
        gem_min_level: g.gem_min_level,
        gem_tier: g.gem_tier,
        str_req_percent: g.str_req_percent,
        dex_req_percent: g.dex_req_percent,
        int_req_percent: g.int_req_percent,
    });

    let currency = imp.currency.map(|c| CurrencyData {
        stack_size: c.stack_size,
        description: c.description,
    });

    let flask = imp.flask.map(|f| FlaskData {
        flask_type: f.flask_type,
        flask_name: f.flask_name,
        flask_life: f.flask_life,
        flask_mana: f.flask_mana,
        flask_recovery_time: f.flask_recovery_time,
    });

    let soul_core = imp.soul_core.map(|s| SoulCoreInfo {
        required_level: s.required_level,
        limit_count: s.limit_count,
        limit_text: s.limit_text,
    });

    let omen = imp.omen.map(|o| OmenInfo {
        description: o.description,
    });

    let breachstone = imp.breachstone.map(|b| BreachstoneInfo {
        tier: b.tier,
        upgrades_to: b.upgrades_to,
        upgrade_currency: b.upgrade_currency,
    });

    let essence = imp.essence.map(|e| EssenceInfo {
        tier: e.tier,
        is_perfect: e.is_perfect,
        upgrade_to_id: e.upgrade_to_id,
        upgrade_to_name: e.upgrade_to_name,
        modifiers: e
            .modifiers
            .into_iter()
            .map(|m| EssenceModifier {
                target_category: m.target_category,
                target_item_classes: m.target_item_classes,
                mod_id: m.mod_id,
                mod_text: m.mod_text,
            })
            .collect(),
    });

    let convert_mod = |m: super::models::ImportedModDisplay| ModDisplay {
        id: m.id,
        text: m.text,
        domain: m.domain,
        slot: m.slot,
        target_item_classes: m.target_item_classes,
    };

    let implicit_mods = imp.implicit_mods.into_iter().map(convert_mod).collect();
    let explicit_mods = imp.explicit_mods.into_iter().map(convert_mod).collect();

    Item {
        id: imp.id,
        name: imp.name,
        is_unique: imp.is_unique,
        unique_name: imp.unique_name,
        base_type: imp.base_type,
        item_class_id: imp.item_class_id,
        category: imp.category,
        rarity_frame: imp.rarity_frame,
        width: imp.width,
        height: imp.height,
        drop_level: imp.drop_level,
        image_url: imp.image_url,
        flavour_text: imp.flavour_text,
        tags: imp.tags,
        is_corrupted: imp.is_corrupted,
        unmodifiable: imp.unmodifiable,
        requirements,
        defences,
        weapon,
        shield,
        gem,
        currency,
        flask,
        soul_core,
        essence,
        omen,
        map_tier: imp.map_tier,
        talisman_tier: imp.talisman_tier,
        breachstone,
        quest_description: imp.quest_description,
        implicit_mods,
        explicit_mods,
    }
}
