#!/usr/bin/env python3
"""
Script to clear all zones from zones.json and start fresh.
"""

import json
import os
import sys
from pathlib import Path

def clear_zones_metadata(zones_file: Path) -> bool:
    """Clear all zones from zones.json and create empty structure."""
    try:
        print(f"Clearing zones from {zones_file.name}...")
        
        # Create empty zones structure
        empty_zones = {
            "zones": {}
        }
        
        # Create backup
        backup_path = zones_file.with_suffix('.json.backup4')
        with open(backup_path, 'w', encoding='utf-8') as f:
            with open(zones_file, 'r', encoding='utf-8') as original:
                backup_data = json.load(original)
                json.dump(backup_data, f, indent=2, ensure_ascii=False)
        print(f"  Created backup: {backup_path.name}")
        
        # Write empty zones structure
        with open(zones_file, 'w', encoding='utf-8') as f:
            json.dump(empty_zones, f, indent=2, ensure_ascii=False)
        
        print(f"  Successfully cleared all zones from zones.json")
        return True
        
    except Exception as e:
        print(f"  Error clearing zones from {zones_file.name}: {e}")
        return False

def main():
    """Main function."""
    print("POE2 Overlord - Clear Zones Metadata Script")
    print("=" * 50)
    
    # Find data directory
    possible_dirs = [
        Path.home() / ".local" / "share" / "poe2-overlord",
        Path.home() / ".config" / "poe2-overlord",
        Path.cwd() / "data"
    ]
    
    data_dir = None
    for dir_path in possible_dirs:
        if dir_path.exists() and (dir_path / "zones.json").exists():
            data_dir = dir_path
            break
    
    if not data_dir:
        print("Error: Could not find POE2 Overlord data directory with zones.json")
        print("Looked in:")
        for dir_path in possible_dirs:
            print(f"  - {dir_path}")
        sys.exit(1)
    
    print(f"Found data directory: {data_dir}")
    
    # Clear zones.json
    zones_file = data_dir / "zones.json"
    if clear_zones_metadata(zones_file):
        print(f"\nClear complete!")
        print("zones.json has been cleared and is ready for fresh zone discovery")
        print("\nBackup file has been created with .backup4 extension")
        print("You can delete it once you've verified the clear worked correctly.")
    else:
        print(f"\nClear failed!")
        sys.exit(1)

if __name__ == "__main__":
    main()
