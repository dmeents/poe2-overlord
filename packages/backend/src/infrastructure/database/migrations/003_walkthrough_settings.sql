ALTER TABLE app_config ADD COLUMN hide_optional_objectives INTEGER NOT NULL DEFAULT 0;
ALTER TABLE app_config ADD COLUMN hide_league_start_objectives INTEGER NOT NULL DEFAULT 0;
ALTER TABLE app_config ADD COLUMN hide_flavor_text INTEGER NOT NULL DEFAULT 0;
ALTER TABLE app_config ADD COLUMN hide_objective_descriptions INTEGER NOT NULL DEFAULT 0;
