# Enhanced Zones Implementation Plan

## Overview
This document outlines the implementation plan for a dynamic zone discovery system that extracts rich wiki data (NPCs, interactables, quests, etc.) from Path of Exile 2 wiki pages as players explore the game. Zone data is built up progressively and stored in the app's config directory alongside character data.

## Project Structure
- **Root**: `/home/david-meents/Documents/projects/poe2-overlord/`
- **Backend**: `packages/backend/`
- **Current zones.json**: `packages/backend/config/zones.json` (static reference data)
- **Dynamic zones.json**: `~/.local/share/poe2-overlord/zones.json` (discovered zone data)
- **Character data**: `~/.local/share/poe2-overlord/characters/` (existing)

## Two-Tier Zone Data System

### Global Zone Data File
- **Location**: `~/.local/share/poe2-overlord/zones.json`
- **Purpose**: Store discovered zone metadata (NPCs, images, descriptions, etc.)
- **Shared**: All characters reference the same zone data
- **Discovery**: Built up as players explore zones

### Character Data Enhancement
- **Location**: Character-specific JSON files
- **Purpose**: Store player-specific zone stats + reference to global zone data
- **Enhancement**: Look up zones in global file and append enhanced data

### Dynamic Zone Discovery Flow

1. **Scene Detection**: Game monitoring detects zone change
2. **Zone Name Extraction**: Parse zone name from scene data
3. **Global Zone Check**: Check if zone exists in `~/.local/share/poe2-overlord/zones.json`
4. **Freshness Check**: Verify global zone data isn't stale (based on `last_wiki_updated`)
5. **Wiki Fetch**: If missing/stale, fetch and parse wiki page → Save to global zones.json
6. **Character Enhancement**: When getting character data, look up each zone in global file
7. **Data Merge**: Append enhanced zone data to character's ZoneStats
8. **UI Update**: Emit `CharacterTrackingDataUpdated` event

## Implementation Phases

### Phase 1: Data Structure Design ✅ COMPLETED
- [x] Design enhanced Zone struct with all fields
- [x] Create supporting structs (Npc, Interactable, Quest, etc.)
- [x] Define enums for type safety
- [x] Plan JSON schema evolution

### Phase 2: Core Infrastructure
- [ ] **2.1**: Create wiki parser module
- [ ] **2.2**: Add HTTP client dependencies
- [ ] **2.3**: Implement basic HTML parsing
- [ ] **2.4**: Create zone data migration utilities
- [ ] **2.5**: Add configuration management

### Phase 3: Wiki Data Extraction
- [ ] **3.1**: Implement basic zone info extraction
- [ ] **3.2**: Add NPC extraction logic
- [ ] **3.3**: Add interactable extraction logic
- [ ] **3.4**: Add quest extraction logic
- [ ] **3.5**: Add item drops extraction
- [ ] **3.6**: Add image URL extraction
- [ ] **3.7**: Add metadata extraction

### Phase 4: Two-Tier System Implementation
- [ ] **4.1**: Implement global zone data service
- [ ] **4.2**: Create zone discovery and storage logic
- [ ] **4.3**: Implement time-based update throttling
- [ ] **4.4**: Add retry logic for failed extractions
- [ ] **4.5**: Integrate with existing game monitoring
- [ ] **4.6**: Enhance character service with zone lookup
- [ ] **4.7**: Update frontend types and hooks

#### 4.1: Implement Global Zone Data Service
**Location**: `packages/backend/src/domain/zone_configuration/service.rs`

**Purpose**: Manage global zone data storage and retrieval

**Implementation**:
```rust
pub struct GlobalZoneService {
    zones_file: PathBuf,
    zones: HashMap<String, Zone>,
    settings: ZoneUpdateSettings,
}

impl GlobalZoneService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let data_dir = get_data_directory()?;
        let zones_file = data_dir.join("zones.json");
        
        // Load existing zones or create empty config
        let config = if zones_file.exists() {
            let content = std::fs::read_to_string(&zones_file)?;
            serde_json::from_str::<ZoneConfig>(&content)?
        } else {
            ZoneConfig {
                zones: Vec::new(),
                settings: ZoneUpdateSettings::default(),
            }
        };
        
        let zones: HashMap<String, Zone> = config.zones
            .into_iter()
            .map(|zone| (zone.id.clone(), zone))
            .collect();
            
        Ok(Self {
            zones_file,
            zones,
            settings: config.settings,
        })
    }
    
    pub async fn get_zone(&self, zone_id: &str) -> Option<&Zone> {
        self.zones.get(zone_id)
    }
    
    pub async fn save_zone(&mut self, zone: Zone) -> Result<(), Box<dyn std::error::Error>> {
        self.zones.insert(zone.id.clone(), zone);
        self.save_to_file().await
    }
    
    pub async fn discover_zone(&mut self, zone_name: &str) -> Result<Option<Zone>, Box<dyn std::error::Error>> {
        // Check if zone already exists and is fresh
        if let Some(existing_zone) = self.zones.get(zone_name) {
            if self.is_zone_fresh(existing_zone) {
                return Ok(Some(existing_zone.clone()));
            }
        }
        
        // Discover zone from wiki
        let wiki_parser = WikiParser::new();
        let wiki_url = format!("https://www.poe2wiki.net/wiki/{}", zone_name);
        
        match wiki_parser.parse_zone_page(&wiki_url).await {
            Ok(mut zone) => {
                zone.id = zone_name.to_string();
                zone.last_wiki_updated = Some(Utc::now());
                self.save_zone(zone.clone()).await?;
                Ok(Some(zone))
            }
            Err(e) => {
                eprintln!("Failed to discover zone {}: {}", zone_name, e);
                Ok(None)
            }
        }
    }
    
    fn is_zone_fresh(&self, zone: &Zone) -> bool {
        if let Some(last_updated) = zone.last_wiki_updated {
            let days_since_update = Utc::now().signed_duration_since(last_updated).num_days();
            days_since_update < self.settings.update_interval_days as i64
        } else {
            false
        }
    }
    
    async fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = ZoneConfig {
            zones: self.zones.values().cloned().collect(),
            settings: self.settings.clone(),
        };
        
        let content = serde_json::to_string_pretty(&config)?;
        std::fs::write(&self.zones_file, content)?;
        Ok(())
    }
}
```

#### 4.2: Create Zone Discovery and Storage Logic
**Location**: `packages/backend/src/domain/zone_configuration/discovery_service.rs`

**Purpose**: Handle zone discovery workflow and integration

**Implementation**:
```rust
pub struct ZoneDiscoveryService {
    global_zone_service: GlobalZoneService,
    wiki_parser: WikiParser,
}

impl ZoneDiscoveryService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            global_zone_service: GlobalZoneService::new().await?,
            wiki_parser: WikiParser::new(),
        })
    }
    
    pub async fn ensure_zone_data(&mut self, zone_name: &str) -> Result<Option<Zone>, Box<dyn std::error::Error>> {
        // Check if zone exists in global data
        if let Some(zone) = self.global_zone_service.get_zone(zone_name).await {
            return Ok(Some(zone.clone()));
        }
        
        // Discover zone from wiki
        self.global_zone_service.discover_zone(zone_name).await
    }
}
```

### Phase 5: Frontend Component Integration
- [ ] **5.1**: Update ZoneCard Component
- [ ] **5.2**: Update Zone Filters (Optional)
- [ ] **5.3**: Update ZoneList Component

### Phase 6: Integration & Testing
- [ ] **6.1**: Initialize Empty Zone Data
- [ ] **6.2**: Add CLI commands for manual updates
- [ ] **6.3**: Create unit tests
- [ ] **6.4**: Performance optimization

## Detailed Implementation Steps

### Phase 2: Core Infrastructure

#### 2.1: Create Wiki Parser Module
**Location**: `packages/backend/src/infrastructure/parsing/wiki_parser.rs`

**Implementation**:
```rust
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct WikiParser {
    base_url: String,
    client: reqwest::Client,
}

impl WikiParser {
    pub fn new() -> Self {
        Self {
            base_url: "https://www.poe2wiki.net".to_string(),
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn parse_zone_page(&self, wiki_url: &str) -> Result<Zone, Box<dyn std::error::Error>> {
        // Implementation details in Phase 3
    }
}
```

#### 2.2: Add HTTP Client Dependencies
**File**: `packages/backend/Cargo.toml`

**Dependencies to add**:
```toml
[dependencies]
scraper = "0.18"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
```

#### 2.3: Implement Basic HTML Parsing
**Location**: `packages/backend/src/infrastructure/parsing/wiki_parser.rs`

**Functions to implement**:
- `extract_basic_info()`
- `extract_npcs()`
- `extract_interactables()`
- `extract_quests()`
- `extract_image_url()`

#### 2.4: Create Zone Data Migration Utilities
**Location**: `packages/backend/src/infrastructure/persistence/zone_migration.rs`

**Purpose**: Convert existing zones.json to new format

#### 2.5: Add Configuration Management
**Location**: `packages/backend/src/domain/configuration/zone_config.rs`

**Configuration structs**:
```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct ZoneUpdateSettings {
    pub update_interval_days: u32,
    pub max_fetch_attempts: u32,
    pub batch_size: usize,
    pub rate_limit_delay_ms: u64,
}

// Global Zone Data Structure (stored in ~/.local/share/poe2-overlord/zones.json)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Zone {
    pub id: String,                    // Unique zone identifier
    pub name: String,                  // Zone display name
    pub location_type: LocationType,   // Zone type (dungeon, town, etc.)
    pub act: Option<String>,           // Act number/name
    pub is_town: bool,                 // Whether this is a town
    pub zone_level: Option<u32>,       // Area level
    
    // Wiki-discovered data
    pub image_url: Option<String>,     // Zone screenshot URL
    pub description: Option<String>,   // Zone description
    pub biome: Option<String>,         // Biome type
    pub area_type: Option<String>,     // Area classification
    pub waypoint: Option<bool>,        // Has waypoint
    pub npcs: Option<Vec<Npc>>,        // NPCs in zone
    pub interactables: Option<Vec<Interactable>>, // Interactable objects
    pub quests: Option<Vec<Quest>>,    // Quests available
    pub special_mechanics: Option<Vec<String>>, // Special mechanics
    pub last_wiki_updated: Option<DateTime<Utc>>, // Last wiki fetch
    pub wiki_url: Option<String>,      // Source wiki URL
}

// Global Zone Configuration (wraps the zones array)
#[derive(Serialize, Deserialize, Debug)]
pub struct ZoneConfig {
    pub zones: Vec<Zone>,
    pub settings: ZoneUpdateSettings,
}
```

### Phase 3: Wiki Data Extraction

#### 3.1: Basic Zone Info Extraction
**Selectors to implement**:
- Zone name: `h1.firstHeading`
- Act: `.infobox tr:contains('Act') td`
- Area level: `.infobox tr:contains('Area level') td`
- Biome: `.infobox tr:contains('Biome') td`
- Waypoint: `.infobox tr:contains('Waypoint') td`

#### 3.2: NPC Extraction Logic
**Selectors**:
- NPC list: `h2:contains('NPCs') + ul li`
- NPC names: `li a[href*="/wiki/"]`
- NPC descriptions: `li` text after name

#### 3.3: Interactable Extraction Logic
**Selectors**:
- Interactables: `h2:contains('Interactables') + ul li`
- Names and descriptions from list items

#### 3.4: Quest Extraction Logic
**Selectors**:
- Quests: `h2:contains('Quests') + ul li`
- Quest names and descriptions

#### 3.5: Item Drops Extraction
**Selectors**:
- Item drops: `h2:contains('Item drops') + ul li`
- Unique items, currency, etc.

#### 3.6: Image URL Extraction
**Selectors**:
- Images: `img[alt*='screenshot']`, `img[alt*='area']`
- Convert relative URLs to absolute

#### 3.7: Metadata Extraction
**Selectors**:
- Version history: `.wikitable tr`
- Categories: `.catlinks a`
- Last edited: `.lastmod`

### Phase 4: Update System

#### 4.1: Time-Based Update Throttling
**Location**: `packages/backend/src/domain/zone_configuration/update_service.rs`

**Logic**:
- Check `last_updated` field
- Only update if `update_interval_days` has passed
- Track `image_fetch_attempts` to prevent infinite retries

#### 4.2: Retry Logic
**Implementation**:
- Exponential backoff for failed requests
- Maximum retry attempts per zone
- Log failed extractions for debugging

#### 4.3: Background Update Service
**Location**: `packages/backend/src/application/zone_update_service.rs`

**Features**:
- Priority queue for zone updates
- Batch processing
- Rate limiting
- Integration with existing service orchestrator

#### 4.4: Game Monitoring Integration
**Location**: `packages/backend/src/domain/game_monitoring/`

**Integration points**:
- `on_scene_change()` - detect zone changes and trigger discovery
- `on_zone_entered()` - queue zone for wiki data extraction
- **Zone name extraction** - convert scene data to wiki-friendly format
- **Wiki URL generation** - create proper URLs for zone lookups

#### 4.5: Enhance Existing Character Tracking System
**Location**: `packages/backend/src/domain/character/`

**Integration Points**:
- Enhance `ZoneStats` in `CharacterData` with wiki data
- Modify `enter_zone()` to trigger wiki discovery
- Use existing `CharacterTrackingDataUpdated` events
- No new Tauri commands needed - use existing character commands

**Two-Tier ZoneStats Structure**:
```rust
// ZoneStats - player-specific data + reference to global zone data
pub struct ZoneStats {
    // Player-specific tracking data only
    pub location_id: String,           // Reference to global zone data
    pub duration: u64,                 // Time spent in zone
    pub deaths: u32,                   // Deaths in this zone
    pub visits: u32,                   // Number of visits
    pub first_visited: String,         // First visit timestamp
    pub last_visited: String,          // Last visit timestamp
    pub is_active: bool,               // Currently in this zone
    pub entry_timestamp: Option<String>, // Current visit start time
    
    // Enhanced zone data (populated from global zones.json)
    pub enhanced_data: Option<Zone>,   // Looked up from global zone data
}

// Note: Zone struct is defined above in the global zone data section
// This contains all the wiki-discovered metadata (NPCs, images, etc.)
```

#### 4.6: Integrate Wiki Discovery into Character Service
**Location**: `packages/backend/src/domain/character/service.rs`

**Modifications**:
- Add wiki discovery service to character service
- Trigger wiki discovery in `enter_zone()` method
- Update `ZoneStats` with enhanced data when available
- Use existing `CharacterTrackingDataUpdated` event

**Implementation**:
```rust
impl CharacterService {
    async fn enter_zone(&self, ...) -> Result<(), AppError> {
        // Existing zone entry logic...
        
        // Check if zone exists in global zones.json
        if let Some(zone_data) = self.global_zone_service.get_zone(&zone_name).await {
            // Zone exists, populate enhanced_data
            zone_stats.enhanced_data = Some(zone_data);
        } else {
            // Zone doesn't exist, trigger wiki discovery
            if let Some(discovered_zone) = self.discover_zone_data(&zone_name).await {
                // Save to global zones.json
                self.global_zone_service.save_zone(discovered_zone.clone()).await?;
                // Populate enhanced_data
                zone_stats.enhanced_data = Some(discovered_zone);
            }
        }
        
        // Existing event emission...
    }
    
    async fn get_character_data(&self, character_id: &str) -> Result<CharacterData, AppError> {
        // Load character data
        let mut character = self.load_character(character_id).await?;
        
        // Enhance zone data by looking up in global zones.json
        for zone_stats in &mut character.zones {
            if let Some(zone_data) = self.global_zone_service.get_zone(&zone_stats.location_id).await {
                zone_stats.enhanced_data = Some(zone_data);
            }
        }
        
        Ok(character)
    }
}
```

#### 4.7: Clean Implementation (No Migration Needed)
**Location**: `packages/backend/src/domain/character/`

**Implementation**:
- Implement new `ZoneStats` structure directly
- No backwards compatibility needed for local development
- Start fresh with refactored data structure
- Wiki discovery will populate `enhanced_data` as zones are visited

#### 4.8: Update Frontend Types (Minimal Changes)
**Location**: `packages/frontend/src/types/character.ts`

**Refactored ZoneStats**:
```typescript
// ZoneStats - player-specific data + reference to global zone data
export interface ZoneStats {
  // Player-specific tracking data only
  location_id: string;           // Reference to global zone data
  duration: number;              // Time spent in zone
  deaths: number;                // Deaths in this zone
  visits: number;                // Number of visits
  first_visited: string;         // First visit timestamp
  last_visited: string;          // Last visit timestamp
  is_active: boolean;            // Currently in this zone
  entry_timestamp?: string;      // Current visit start time
  
  // Enhanced zone data (populated from global zones.json)
  enhanced_data?: Zone;          // Looked up from global zone data
}

// Zone - global zone data structure (matches Rust Zone struct)
export interface Zone {
  id: string;                    // Unique zone identifier
  name: string;                  // Zone display name
  location_type: LocationType;   // Zone type (dungeon, town, etc.)
  act?: string;                  // Act number/name
  is_town: boolean;              // Whether this is a town
  zone_level?: number;           // Area level
  
  // Wiki-discovered data
  image_url?: string;            // Zone screenshot URL
  description?: string;          // Zone description
  biome?: string;                // Biome type
  area_type?: string;            // Area classification
  waypoint?: boolean;            // Has waypoint
  npcs?: Npc[];                  // NPCs in zone
  interactables?: Interactable[]; // Interactable objects
  quests?: Quest[];              // Quests available
  special_mechanics?: string[];  // Special mechanics
  last_wiki_updated?: string;    // Last wiki fetch
  wiki_url?: string;             // Source wiki URL
}

export interface Npc {
  name: string;
  npc_type: 'Vendor' | 'QuestGiver' | 'Crafting' | 'Special';
  services: string[];
  description?: string;
  wiki_url?: string;
}

export interface Interactable {
  name: string;
  interactable_type: 'Utility' | 'Transport' | 'Crafting' | 'Vendor' | 'Quest';
  description: string;
  unlock_requirement?: string;
  wiki_url?: string;
}
```

### Phase 5: Frontend Component Integration

#### 5.1: Update ZoneCard Component
**Location**: `packages/frontend/src/components/zones/zone-card/zone-card.tsx`

**Enhancements**:
- Display zone name from `zone.enhanced_data?.name` (looked up from global zones.json)
- Display zone images when `zone.enhanced_data?.image_url` is available
- Show NPCs, interactables, and quests from `zone.enhanced_data`
- Add wiki link integration using `zone.enhanced_data?.wiki_url`
- Display enhanced zone metadata (biome, area type, etc.)
- Handle cases where `enhanced_data` might be `undefined` (zone not yet discovered)

**Implementation**:
```typescript
// No new props needed - use existing zone prop
interface ZoneCardProps {
  zone: ZoneStats; // Contains enhanced_data from global zone lookup
  className?: string;
}

// In component:
const { enhanced_data } = zone;
// enhanced_data is populated by looking up zone in global zones.json
const zoneName = enhanced_data?.name || zone.location_id; // Fallback to location_id
const isTown = enhanced_data?.is_town || false;
const act = enhanced_data?.act;

if (enhanced_data?.image_url) {
  // Display zone image
}
if (enhanced_data?.npcs) {
  // Display NPCs
}
```

#### 5.2: Update Zone Filters (Optional)
**Location**: `packages/frontend/src/hooks/useZoneFilters.ts`

**New Filter Options** (only if needed):
```typescript
export interface ZoneFilters {
  // Existing filters...
  hasEnhancedData: boolean | null; // Filter zones with wiki data
  biome: string | 'All'; // Filter by biome from enhanced_data
  areaType: string | 'All'; // Filter by area type
}
```

#### 5.3: Update ZoneList Component
**Location**: `packages/frontend/src/components/zones/zone-list/zone-list.tsx`

**Enhancements**:
- Pass enhanced data to ZoneCard components
- No structural changes needed - existing props work

### Phase 6: Integration & Testing

#### 6.1: Initialize Empty Zone Data
**Location**: `packages/backend/src/infrastructure/persistence/zone_storage.rs`

**Process**:
1. Create empty zones.json in `~/.local/share/poe2-overlord/`
2. Initialize with default settings
3. Let dynamic discovery populate data as player explores

**Initialization Logic**:
```rust
pub fn initialize_zone_data() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = get_data_directory()?;
    let zones_file = data_dir.join("zones.json");
    
    if !zones_file.exists() {
        let initial_config = ZoneConfig {
            zones: Vec::new(),
            settings: ZoneUpdateSettings::default(),
        };
        
        std::fs::create_dir_all(&data_dir)?;
        std::fs::write(&zones_file, serde_json::to_string_pretty(&initial_config)?)?;
    }
    
    Ok(())
}
```

#### 5.2: CLI Commands
**Location**: `packages/backend/src/infrastructure/cli/zone_commands.rs`

**Commands**:
- `discover-zone <zone_name>` - Manually discover a specific zone
- `clear-zone-data` - Clear all discovered zone data (for testing)
- `list-discovered-zones` - Show all discovered zones

#### 5.3: Unit Tests
**Location**: `packages/backend/src/infrastructure/parsing/tests/`

**Test files**:
- `wiki_parser_tests.rs`
- `zone_discovery_tests.rs`
- `zone_storage_tests.rs`

#### 5.4: Performance Optimization
**Optimizations**:
- Connection pooling for HTTP requests
- Caching parsed HTML
- Parallel processing of multiple zones
- Memory usage optimization

## File Structure

```
packages/backend/src/
├── domain/
│   ├── zone_configuration/
│   │   ├── mod.rs
│   │   ├── models.rs          # Enhanced Zone structs
│   │   ├── repository.rs      # Zone data access (~/.local/share/poe2-overlord/)
│   │   ├── service.rs         # Zone business logic
│   │   └── discovery_service.rs # Dynamic zone discovery
│   └── configuration/
│       └── zone_config.rs     # Update settings
├── infrastructure/
│   ├── parsing/
│   │   ├── mod.rs
│   │   ├── wiki_parser.rs     # Wiki data extraction
│   │   └── scene_parser.rs    # Zone name extraction from scenes
│   ├── persistence/
│   │   └── zone_storage.rs    # Zone data storage management
│   └── cli/
│       └── zone_commands.rs   # CLI commands
└── application/
    └── zone_discovery_service.rs # Background discovery service
```

## Data Directory Structure

```
~/.local/share/poe2-overlord/
├── characters/
│   ├── character1.json
│   └── character2.json
├── zones.json                 # Discovered zone data
├── logs/
└── cache/
    └── wiki/                  # Cached wiki pages
```

## Config Directory Structure

```
~/.config/poe2-overlord/
├── settings.json              # User preferences
└── app_config.json           # Application configuration
```

## Directory Creation Implementation

### Data Directory Setup
**Location**: `packages/backend/src/infrastructure/persistence/directory_manager.rs`

**Purpose**: Handle creation and management of application directories

**Implementation**:
```rust
use std::path::PathBuf;
use std::fs;

pub fn get_data_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?;
    
    let data_dir = home_dir.join(".local/share/poe2-overlord");
    
    // Create directory structure if it doesn't exist
    create_directory_structure(&data_dir)?;
    
    Ok(data_dir)
}

pub fn get_config_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?;
    
    let config_dir = home_dir.join(".config/poe2-overlord");
    
    // Create directory structure if it doesn't exist
    create_directory_structure(&config_dir)?;
    
    Ok(config_dir)
}

fn create_directory_structure(base_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Create base directory
    fs::create_dir_all(base_dir)?;
    
    // Create subdirectories
    let subdirs = vec![
        "characters",
        "logs", 
        "cache",
        "cache/wiki",
    ];
    
    for subdir in subdirs {
        let path = base_dir.join(subdir);
        fs::create_dir_all(&path)?;
    }
    
    Ok(())
}

pub fn ensure_directories_exist() -> Result<(), Box<dyn std::error::Error>> {
    get_data_directory()?;
    get_config_directory()?;
    Ok(())
}
```

### Dependencies Required
**File**: `packages/backend/Cargo.toml`

**Add to dependencies**:
```toml
[dependencies]
dirs = "5.0"  # For cross-platform directory detection
```

### Integration Points
**Location**: `packages/backend/src/main.rs` or application startup

**Call during app initialization**:
```rust
// Ensure all required directories exist
directory_manager::ensure_directories_exist()?;

// Initialize zone data
zone_storage::initialize_zone_data()?;
```

## Configuration Files

### Enhanced zones.json Structure
```json
{
  "zones": [
    {
      "id": "P3_Town",
      "name": "The Glade",
      "act": 6,
      "area_level": 64,
      "biome": "Interlude",
      "area_type": "Town",
      "waypoint": true,
      "connections": [...],
      "description": "...",
      "image_url": "...",
      "last_updated": "2024-01-15T10:30:00Z",
      "image_fetch_attempts": 0,
      "wiki_url": "...",
      "version_introduced": "0.3.0",
      "npcs": [...],
      "interactables": [...],
      "quests": [...],
      "item_drops": {...},
      "special_mechanics": [...],
      "encounters": [...],
      "bosses": [...],
      "pinnacle_encounters": [...],
      "metadata": {...}
    }
  ],
  "settings": {
    "update_interval_days": 7,
    "max_fetch_attempts": 3,
    "batch_size": 5,
    "rate_limit_delay_ms": 1000
  }
}
```

### Migration from Current Structure
The current nested structure:
```json
{
  "acts": [
    {
      "act_name": "Act 1",
      "act_number": 1,
      "zones": [
        {
          "zone_name": "Clearfell Encampment",
          "is_town": true
        }
      ]
    }
  ]
}
```

Will be flattened to:
```json
{
  "zones": [
    {
      "name": "Clearfell Encampment",
      "is_town": true,
      "act": 1,
      "act_name": "Act 1",
      // ... other enhanced fields
    }
  ],
  "settings": { ... }
}
```

## Benefits of Refactored ZoneStats Structure

### **Separation of Concerns**:
1. **Player Data**: `ZoneStats` contains only player-specific tracking data
2. **Zone Data**: `EnhancedZoneData` contains all zone metadata and wiki data
3. **Clean Reference**: `location_id` serves as the link between them

### **Data Efficiency**:
1. **No Duplication**: Zone metadata stored once in `enhanced_data`
2. **Smaller Payloads**: Player stats are lightweight
3. **Better Caching**: Zone data can be cached separately from player data

### **Maintainability**:
1. **Single Source of Truth**: Zone metadata comes from wiki discovery
2. **Consistent Data**: All zones have the same metadata structure
3. **Easy Updates**: Wiki data updates automatically refresh zone info

### **Performance Benefits**:
1. **Lazy Loading**: Zone metadata only loaded when needed
2. **Selective Updates**: Only player stats change frequently
3. **Memory Efficient**: Zone data shared across characters

### **Developer Experience**:
1. **Clear Structure**: Easy to understand what data belongs where
2. **Type Safety**: Compile-time validation of data structure
3. **Future-Proof**: Easy to add new zone metadata fields

## Data Flow and Integration Points

### Backend to Frontend Data Flow

1. **Zone Discovery Trigger**:
   - Game monitoring detects scene change
   - Character service `enter_zone()` called
   - Zone name extracted from scene data
   - Wiki discovery triggered for enhanced data

2. **Data Processing**:
   - Wiki page fetched and parsed
   - Enhanced zone data created
   - `ZoneStats.enhanced_data` updated in character data
   - Character data saved to `~/.local/share/poe2-overlord/characters/`
   - Existing `CharacterTrackingDataUpdated` event emitted

3. **Frontend Updates**:
   - Existing `useCharacterManagement()` hook receives character update
   - Zone data automatically updated in component state
   - UI components re-render with enhanced zone data
   - ZoneCard displays enhanced information when available

### Key Integration Points

#### **Existing Systems to Integrate With**:

1. **Character Tracking System** (Primary Integration):
   - Enhance `ZoneStats` with `enhanced_data` field
   - Wiki discovery triggered in `enter_zone()` method
   - Use existing `CharacterTrackingDataUpdated` events
   - No new Tauri commands needed

2. **Game Monitoring System**:
   - Scene change detection already triggers character zone entry
   - Zone name extraction already working
   - No changes needed to existing flow

3. **Event System**:
   - Use existing `CharacterTrackingDataUpdated` events
   - No new events needed
   - Frontend already listens to character updates

4. **Tauri Commands**:
   - Use existing character commands (`get_character_tracking_data`)
   - No new commands needed
   - Existing error handling works

#### **Frontend Components to Update**:

1. **ZoneCard Component** (Minimal Changes):
   - Display `zone.enhanced_data` when available
   - Show images, NPCs, interactables from enhanced data
   - Wiki link integration using enhanced data

2. **Zone Filters** (Optional):
   - Add filter for zones with enhanced data
   - Filter by biome, area type from enhanced data
   - Minimal changes to existing filter system

3. **Playtime Page** (No Changes):
   - Already displays zone data from character
   - Enhanced data automatically available
   - No structural changes needed

4. **Character Management** (No Changes):
   - Zone data already in character context
   - Enhanced data automatically included
   - No additional integration needed

## Progress Tracking

### Current Status: Phase 1 Complete
- [x] Data structure design
- [x] JSON schema planning
- [x] Implementation plan created

### Next Steps: Phase 2.1
- [ ] Create wiki parser module structure
- [ ] Add required dependencies to Cargo.toml
- [ ] Implement basic HTTP client setup

## Notes
- All new code should follow existing project patterns
- Use existing error handling patterns
- Maintain backward compatibility with current zones.json
- Add comprehensive logging for debugging
- Consider rate limiting for wiki requests

## Dependencies
- `scraper` - HTML parsing
- `reqwest` - HTTP client
- `chrono` - Date/time handling
- `serde` - Serialization (already present)
- `tokio` - Async runtime (already present)

