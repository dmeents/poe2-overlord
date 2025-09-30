# Hooks Directory

This directory contains all React hooks used throughout the POE2 Overlord application. The hooks have been refactored and organized for better maintainability, performance, and reusability.

## Architecture Overview

The hooks are organized into several categories:

### Core Data Hooks

- **`useCharacterData`** - Character data fetching and local state management
- **`useCharacterMutations`** - Character CRUD operations using React Query
- **`useCharacterEvents`** - Tauri event listeners for character data updates
- **`useCharacterManagement`** - Main character management hook (composes focused hooks)

### Query Hooks

- **`useCharacterQueries`** - React Query hooks for character data
- **`useWalkthroughGuide`** - Walkthrough guide data fetching

### Event Hooks

- **`useTauriEventListener`** - Generic Tauri event listener
- **`useMultiTauriEventListener`** - Multiple Tauri event listeners
- **`useGameProcessEvents`** - Game process monitoring events
- **`useServerStatusEvents`** - Server status monitoring events
- **`useWalkthroughEvents`** - Walkthrough progress events

### Filtering & State Hooks

- **`useDataFiltering`** - Generic data filtering and sorting
- **`useFilterState`** - Generic filter and sort state management
- **`useCharacterDataFiltering`** - Character-specific filtering
- **`useZoneDataFiltering`** - Zone-specific filtering
- **`useCharacterFilterState`** - Character filter state
- **`useZoneFilterState`** - Zone filter state

### Utility Hooks

- **`useErrorHandling`** - Standardized error handling
- **`useErrorBoundary`** - React error boundary functionality
- **`useCacheInvalidation`** - React Query cache invalidation
- **`useCharacterConfig`** - Character configuration management
- **`useCRUDOperations`** - Generic CRUD operations

## Key Features

### 1. Modular Architecture

Hooks are broken down into focused, single-responsibility modules:

- **Data hooks** handle data fetching and state
- **Mutation hooks** handle data modifications
- **Event hooks** handle real-time updates
- **Utility hooks** provide common functionality

### 2. Type Safety

All hooks are fully typed with TypeScript:

- Generic types for reusability
- Proper error type definitions
- Comprehensive JSDoc documentation

### 3. Error Handling

Standardized error handling across all hooks:

- Consistent error types and messages
- User-friendly error recovery
- React error boundary integration

### 4. Performance Optimization

- React Query for efficient data caching
- Automatic cache invalidation on events
- Optimized re-renders with proper dependencies
- Dead code elimination

## Usage Examples

### Basic Character Management

```typescript
import { useCharacterManagement } from './hooks';

function CharacterComponent() {
  const {
    characters,
    activeCharacter,
    isLoading,
    error,
    createCharacter,
    updateCharacter,
    deleteCharacter
  } = useCharacterManagement();

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div>
      {characters.map(character => (
        <div key={character.id}>
          {character.name} - {character.class}
        </div>
      ))}
    </div>
  );
}
```

### Event Listening

```typescript
import { useTauriEventListener } from './hooks';

function EventComponent() {
  const { isListening, error } = useTauriEventListener({
    eventName: 'character-updated',
    handler: (data) => {
      console.log('Character updated:', data);
    },
    enabled: true
  });

  return (
    <div>
      Status: {isListening ? 'Listening' : 'Not listening'}
      {error && <div>Error: {error}</div>}
    </div>
  );
}
```

### Data Filtering

```typescript
import { useCharacterDataFiltering } from './hooks';

function FilteredCharacterList() {
  const { filteredData, summary } = useCharacterDataFiltering({
    characters: allCharacters,
    filters: { search: 'wizard', class: 'Witch' },
    sort: { field: 'name', direction: 'asc' }
  });

  return (
    <div>
      <div>Found {filteredData.length} characters</div>
      {filteredData.map(character => (
        <div key={character.id}>{character.name}</div>
      ))}
    </div>
  );
}
```

## Architecture Overview

The hooks use a modular architecture where main hooks compose focused hooks internally:

```typescript
// Main hook composes focused hooks internally
const characterManagement = useCharacterManagement();

// Or use focused hooks directly for more control
const characterData = useCharacterData();
const characterMutations = useCharacterMutations();
const characterEvents = useCharacterEvents(activeCharacterId, setters);
```

### Backward Compatibility

All main hooks maintain full backward compatibility:

- Same public API
- Same return values
- Same function signatures
- No breaking changes for existing components

## Best Practices

### 1. Use Focused Hooks

Prefer focused hooks over monolithic ones:

```typescript
// Good: Focused responsibility
const { characters, isLoading } = useCharacterData();
const { createCharacter } = useCharacterMutations();

// Avoid: Monolithic hook unless needed
const characterManagement = useCharacterManagement();
```

### 2. Error Handling

Always handle errors appropriately:

```typescript
const { data, error, isLoading } = useCharacterData();

if (error) {
  return <ErrorBoundary error={error} />;
}
```

### 3. Event Cleanup

Ensure proper event cleanup:

```typescript
const { isListening } = useTauriEventListener({
  eventName: 'my-event',
  handler: handleEvent,
  enabled: shouldListen, // Control with state
});
```

### 4. Type Safety

Use proper TypeScript types:

```typescript
const { updateCharacter } = useCharacterMutations();
// updateCharacter is properly typed with CharacterFormData
```

## Testing

All hooks include comprehensive tests:

- Unit tests for individual hooks
- Integration tests for composed hooks
- Error scenario testing
- Performance testing

Run tests with:

```bash
yarn test
```

## Contributing

When adding new hooks:

1. **Single Responsibility**: Each hook should have one clear purpose
2. **Type Safety**: Use proper TypeScript types
3. **Error Handling**: Use standardized error handling
4. **Documentation**: Include comprehensive JSDoc comments
5. **Tests**: Add unit tests for new hooks
6. **Examples**: Update usage examples

## Troubleshooting

### Common Issues

1. **Hook Dependencies**: Ensure all dependencies are included in dependency arrays
2. **Event Cleanup**: Make sure event listeners are properly cleaned up
3. **Type Errors**: Use proper TypeScript types for all parameters
4. **Error Handling**: Always handle errors appropriately

### Debug Mode

Enable debug mode for detailed logging:

```typescript
const { data, error } = useCharacterData();
// Check browser console for detailed error information
```

## Performance Considerations

- **React Query**: Leverages caching and background updates
- **Event Optimization**: Events are debounced and optimized
- **Re-render Prevention**: Proper dependency arrays prevent unnecessary re-renders
- **Memory Management**: Automatic cleanup of event listeners and subscriptions

## Future Improvements

- [ ] Add more generic utility hooks
- [ ] Implement hook composition patterns
- [ ] Add performance monitoring
- [ ] Enhance error recovery mechanisms
- [ ] Add more comprehensive tests
