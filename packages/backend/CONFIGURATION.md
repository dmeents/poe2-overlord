# Configuration System

The POE2 Overlord backend includes a comprehensive configuration system that allows users to customize application behavior and settings.

## Configuration Structure

The configuration is stored in a JSON file located at:
- **Linux**: `~/.config/poe2-overlord/config.json`
- **macOS**: `~/Library/Application Support/poe2-overlord/config.json`
- **Windows**: `%APPDATA%\poe2-overlord\config.json`

## Configuration Fields

### `poe_client_log_path` (String)
- **Description**: Path to the POE2 client.txt log file
- **Default**: Empty string
- **Example**: `"C:\\Games\\Path of Exile\\logs\\client.txt"`

### `auto_start_monitoring` (Boolean)
- **Description**: Whether to automatically start process monitoring when the app launches
- **Default**: `false`
- **Example**: `true`

### `log_level` (String)
- **Description**: Application log level
- **Default**: `"info"`
- **Valid Values**: `"debug"`, `"info"`, `"warn"`, `"error"`

## Frontend Usage

The configuration can be accessed and modified from the frontend using Tauri commands:

### Get Current Configuration
```typescript
import { invoke } from '@tauri-apps/api/tauri';

const config = await invoke('get_config');
console.log('Current config:', config);
```

### Update Specific Fields
```typescript
// Set POE client log path
await invoke('set_poe_client_log_path', { path: '/path/to/client.txt' });

// Enable auto-start monitoring
await invoke('set_auto_start_monitoring', { enabled: true });

// Set log level
await invoke('set_log_level', { level: 'debug' });
```

### Update Entire Configuration
```typescript
const newConfig = {
  poe_client_log_path: '/path/to/client.txt',
  auto_start_monitoring: true,
  log_level: 'debug'
};

await invoke('update_config', { newConfig });
```

### Reset to Defaults
```typescript
await invoke('reset_config_to_defaults');
```

## Backend Implementation

The configuration system consists of:

1. **`AppConfig` Model** (`src/models/mod.rs`): Defines the configuration structure
2. **`ConfigService`** (`src/services/config.rs`): Manages configuration loading, saving, and updates
3. **Configuration Commands** (`src/commands/config_commands.rs`): Tauri commands for frontend interaction

## Automatic Configuration

- Configuration is automatically loaded when the application starts
- If no configuration file exists, default values are used and a new file is created
- Configuration changes are automatically persisted to disk
- The configuration service is thread-safe and can be accessed from multiple parts of the application

## Error Handling

All configuration operations return `Result` types:
- Success operations return the requested data or `()`
- Error operations return a descriptive error message as a string
- Configuration file corruption or permission issues are handled gracefully with fallback to defaults

## Testing

The configuration system includes comprehensive tests located in `tests/config_service_tests.rs`:

```bash
# Run all tests
cargo test

# Run only configuration tests
cargo test --test config_service_tests
```

Tests cover:
- Default configuration creation
- Configuration save/load operations
- Field-specific updates
- Error handling scenarios
- Thread safety and concurrent access
- Configuration persistence and file I/O
