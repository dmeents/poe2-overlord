# Hooks Migration Guide

This guide helps you migrate from the old monolithic hook architecture to the new modular, focused hook system.

## Overview of Changes

The hooks refactor introduces several key improvements:

1. **Modular Architecture**: Large hooks are broken down into focused, single-responsibility hooks
2. **Better Type Safety**: Improved TypeScript types throughout
3. **Standardized Error Handling**: Consistent error patterns across all hooks
4. **Performance Optimizations**: Better caching and re-render prevention
5. **Backward Compatibility**: Main hooks maintain their public API

## Migration Steps

### 1. Character Management Hooks

#### Before (Old Architecture)
```typescript
// Single large hook with multiple responsibilities
const {
  characters,
  activeCharacter,
  isLoading,
  error,
  createCharacter,
  updateCharacter,
  deleteCharacter,
  loadCharacters,
  loadActiveCharacter,
  setActiveCharacterId,
  // ... many more functions
} = useCharacterManagement();
```

#### After (New Architecture)
```typescript
// Option 1: Use the main hook (backward compatible)
const characterManagement = useCharacterManagement();
// Same API as before, but internally uses focused hooks

// Option 2: Use focused hooks directly (recommended for new code)
const characterData = useCharacterData();
const characterMutations = useCharacterMutations();
const characterEvents = useCharacterEvents(activeCharacterId, setters);
```

### 2. Error Handling Migration

#### Before (Inconsistent Error Handling)
```typescript
// Different error handling patterns across hooks
const [error, setError] = useState<string | null>(null);

try {
  const result = await someOperation();
} catch (err) {
  setError(err.message);
}
```

#### After (Standardized Error Handling)
```typescript
// Consistent error handling with useErrorHandling
const { error, handleError, clearError, handleAsyncOperation } = useErrorHandling({
  enableLogging: true,
  enableRecovery: true
});

// Option 1: Manual error handling
try {
  const result = await someOperation();
} catch (err) {
  handleError(err, 'operation context');
}

// Option 2: Automatic error handling
const result = await handleAsyncOperation(
  () => someOperation(),
  'operation context'
);
```

### 3. Event Listening Migration

#### Before (Manual Event Management)
```typescript
// Manual event listener setup and cleanup
useEffect(() => {
  const unlisten = listen('my-event', (event) => {
    // Handle event
  });

  return () => {
    unlisten();
  };
}, []);
```

#### After (Standardized Event Hooks)
```typescript
// Use useTauriEventListener for single events
const { isListening, error } = useTauriEventListener({
  eventName: 'my-event',
  handler: (data) => {
    // Handle event
  },
  enabled: true
});

// Use useMultiTauriEventListener for multiple events
const { isListening, errors } = useMultiTauriEventListener({
  listeners: [
    { eventName: 'event1', handler: handleEvent1 },
    { eventName: 'event2', handler: handleEvent2 }
  ],
  enabled: true
});
```

### 4. Data Filtering Migration

#### Before (Duplicate Filtering Logic)
```typescript
// Duplicate filtering logic in multiple components
const [filteredData, setFilteredData] = useState([]);
const [sort, setSort] = useState({ field: 'name', direction: 'asc' });

useEffect(() => {
  let filtered = data.filter(item => {
    // Complex filtering logic
  });
  
  filtered.sort((a, b) => {
    // Complex sorting logic
  });
  
  setFilteredData(filtered);
}, [data, filters, sort]);
```

#### After (Generic Filtering Hooks)
```typescript
// Use generic useDataFiltering hook
const { filteredData, summary } = useDataFiltering({
  data: allData,
  filters: filterState,
  sort: sortState,
  filterFunction: createFilterFunction(),
  sortFunction: createSortFunction(),
  summaryFunction: createSummaryFunction()
});

// Or use specific hooks for common data types
const { filteredData, summary } = useCharacterDataFiltering({
  characters: allCharacters,
  filters: characterFilters,
  sort: characterSort
});
```

## Breaking Changes

### 1. Hook Dependencies
Some hooks now have additional dependencies that need to be included:

```typescript
// Before
useEffect(() => {
  // Some effect
}, [dependency1, dependency2]);

// After - may need additional dependencies
useEffect(() => {
  // Some effect
}, [dependency1, dependency2, newDependency]);
```

### 2. Error Object Structure
Error objects now have a standardized structure:

```typescript
// Before
const error: string | null = null;

// After
const error: StandardError | null = null;
// error.type, error.message, error.timestamp, etc.
```

### 3. Event Handler Signatures
Event handlers now have more specific type signatures:

```typescript
// Before
const handler = (event: any) => {
  // Handle event
};

// After
const handler = (data: SpecificEventType) => {
  // Handle event with proper typing
};
```

## Migration Checklist

### Phase 1: Update Imports
- [ ] Update import statements to use new hook names
- [ ] Remove unused imports
- [ ] Add new required imports

### Phase 2: Update Hook Usage
- [ ] Replace old hook calls with new focused hooks
- [ ] Update error handling to use standardized patterns
- [ ] Update event listeners to use new event hooks

### Phase 3: Update Error Handling
- [ ] Replace manual error state with useErrorHandling
- [ ] Update error display components
- [ ] Add error recovery mechanisms

### Phase 4: Update Filtering
- [ ] Replace custom filtering logic with generic hooks
- [ ] Update filter state management
- [ ] Test filtering performance

### Phase 5: Testing
- [ ] Update unit tests for new hook structure
- [ ] Test error scenarios
- [ ] Test event handling
- [ ] Test filtering functionality

## Common Issues and Solutions

### Issue: Hook Dependencies Missing
**Problem**: React Hook exhaustive-deps warnings
**Solution**: Add missing dependencies to dependency arrays

```typescript
// Add missing dependencies
useEffect(() => {
  // Effect logic
}, [dependency1, dependency2, missingDependency]);
```

### Issue: Type Errors
**Problem**: TypeScript errors with new hook types
**Solution**: Update type annotations and use proper types

```typescript
// Use proper types
const { error }: { error: StandardError | null } = useErrorHandling();
```

### Issue: Event Cleanup
**Problem**: Event listeners not being cleaned up
**Solution**: Use the new event hooks that handle cleanup automatically

```typescript
// Use useTauriEventListener instead of manual setup
const { isListening } = useTauriEventListener({
  eventName: 'my-event',
  handler: handleEvent,
  enabled: true
});
```

### Issue: Performance Issues
**Problem**: Unnecessary re-renders or slow filtering
**Solution**: Use the optimized hooks and proper dependency arrays

```typescript
// Use optimized filtering hooks
const { filteredData } = useCharacterDataFiltering({
  characters,
  filters,
  sort
});
```

## Testing Migration

### Unit Tests
Update unit tests to work with new hook structure:

```typescript
// Before
const { result } = renderHook(() => useCharacterManagement());

// After
const { result } = renderHook(() => useCharacterData());
const { result: mutations } = renderHook(() => useCharacterMutations());
```

### Integration Tests
Test the composed hooks work together:

```typescript
// Test that composed hooks work together
const { result } = renderHook(() => useCharacterManagement());
expect(result.current.characters).toBeDefined();
expect(result.current.createCharacter).toBeDefined();
```

## Performance Considerations

### Before Migration
- Large hooks with multiple responsibilities
- Inconsistent error handling
- Manual event management
- Duplicate filtering logic

### After Migration
- Focused hooks with single responsibilities
- Standardized error handling
- Automatic event management
- Reusable filtering logic
- Better TypeScript types
- Optimized re-renders

## Rollback Plan

If issues arise during migration:

1. **Immediate Rollback**: Revert to previous commit
2. **Partial Rollback**: Revert specific hook changes
3. **Gradual Migration**: Migrate one hook at a time
4. **Hybrid Approach**: Use new hooks alongside old ones

## Support

For questions or issues during migration:

1. Check the hooks README for detailed documentation
2. Review the JSDoc comments in individual hook files
3. Look at the usage examples in the README
4. Test with the provided unit tests

## Next Steps

After completing migration:

1. **Remove Old Code**: Clean up unused old hook implementations
2. **Update Documentation**: Update component documentation
3. **Performance Testing**: Run performance tests
4. **Code Review**: Review migrated code for best practices
5. **Team Training**: Train team on new hook patterns
