#!/usr/bin/env python3
"""
Migration script to convert existing character data from old zone format to new format.

Old format:
- location_id, location_name, location_type, act, is_town, zone_level

New format:
- area_id (derived from location_id or location_name)
- duration, deaths, visits, first_visited, last_visited, is_active, entry_timestamp (unchanged)
"""

import json
import os
import sys
from pathlib import Path
from typing import Dict, Any, List
import shutil
from datetime import datetime

def convert_zone_stats(old_zone: Dict[str, Any]) -> Dict[str, Any]:
    """Convert old zone stats format to new format."""
    
    # Extract area_id from location_id or location_name
    area_id = old_zone.get('location_id', '')
    if not area_id:
        # Fallback to location_name if location_id is empty
        area_id = old_zone.get('location_name', 'unknown_zone')
        # Convert to snake_case for consistency
        area_id = area_id.lower().replace(' ', '_').replace('-', '_')
    
    # Create new zone stats with only the required fields
    new_zone = {
        'area_id': area_id,
        'duration': old_zone.get('duration', 0),
        'deaths': old_zone.get('deaths', 0),
        'visits': old_zone.get('visits', 0),
        'first_visited': old_zone.get('first_visited', '2025-01-01T00:00:00Z'),
        'last_visited': old_zone.get('last_visited', '2025-01-01T00:00:00Z'),
        'is_active': old_zone.get('is_active', False),
        'entry_timestamp': old_zone.get('entry_timestamp')
    }
    
    return new_zone

def migrate_character_file(file_path: Path) -> bool:
    """Migrate a single character data file."""
    try:
        print(f"Migrating {file_path.name}...")
        
        # Read the existing file
        with open(file_path, 'r', encoding='utf-8') as f:
            character_data = json.load(f)
        
        # Check if zones exist and need migration
        if 'zones' not in character_data:
            print(f"  No zones found in {file_path.name}")
            return True
        
        # Check if already migrated (has area_id field)
        if character_data['zones'] and 'area_id' in character_data['zones'][0]:
            print(f"  {file_path.name} already migrated")
            return True
        
        # Convert zones
        old_zones = character_data['zones']
        new_zones = []
        
        for old_zone in old_zones:
            new_zone = convert_zone_stats(old_zone)
            new_zones.append(new_zone)
        
        # Update character data
        character_data['zones'] = new_zones
        
        # Create backup
        backup_path = file_path.with_suffix('.json.backup')
        shutil.copy2(file_path, backup_path)
        print(f"  Created backup: {backup_path.name}")
        
        # Write migrated data
        with open(file_path, 'w', encoding='utf-8') as f:
            json.dump(character_data, f, indent=2, ensure_ascii=False)
        
        print(f"  Successfully migrated {len(new_zones)} zones")
        return True
        
    except Exception as e:
        print(f"  Error migrating {file_path.name}: {e}")
        return False

def find_character_data_files(data_dir: Path) -> List[Path]:
    """Find all character data files in the data directory."""
    pattern = "character_data_*.json"
    return list(data_dir.glob(pattern))

def main():
    """Main migration function."""
    print("POE2 Overlord - Zone Data Migration Script")
    print("=" * 50)
    
    # Find data directory
    possible_dirs = [
        Path.home() / ".local" / "share" / "poe2-overlord",
        Path.home() / ".config" / "poe2-overlord",
        Path.cwd() / "data"
    ]
    
    data_dir = None
    for dir_path in possible_dirs:
        if dir_path.exists() and (dir_path / "characters.json").exists():
            data_dir = dir_path
            break
    
    if not data_dir:
        print("Error: Could not find POE2 Overlord data directory")
        print("Looked in:")
        for dir_path in possible_dirs:
            print(f"  - {dir_path}")
        sys.exit(1)
    
    print(f"Found data directory: {data_dir}")
    
    # Find character data files
    character_files = find_character_data_files(data_dir)
    
    if not character_files:
        print("No character data files found to migrate")
        return
    
    print(f"Found {len(character_files)} character data files")
    
    # Auto-proceed with migration (for non-interactive use)
    print(f"\nProceeding with migration of {len(character_files)} files...")
    
    # Migrate files
    success_count = 0
    for file_path in character_files:
        if migrate_character_file(file_path):
            success_count += 1
    
    print(f"\nMigration complete!")
    print(f"Successfully migrated: {success_count}/{len(character_files)} files")
    
    if success_count < len(character_files):
        print("Some files failed to migrate. Check the output above for errors.")
        sys.exit(1)
    else:
        print("All files migrated successfully!")
        print("\nBackup files have been created with .backup extension")
        print("You can delete them once you've verified the migration worked correctly.")

if __name__ == "__main__":
    main()
