# Code Patterns

Common patterns and idioms used in POE2 Overlord.

## Component Structure

```tsx
// {name}.component.tsx
import { z } from 'zod';

// Zod schema for props
const propsSchema = z.object({
  characterId: z.string(),
  onSelect: z.function().args(z.string()).returns(z.void())
});

type CharacterCardProps = z.infer<typeof propsSchema>;

/**
 * Displays character information in a card format
 * @param characterId - Unique character identifier
 * @param onSelect - Callback when character is selected
 */
export function CharacterCard({ characterId, onSelect }: CharacterCardProps) {
  // Tanstack Query for data fetching
  const { data, isLoading, error } = useQuery({
    queryKey: ['character', characterId],
    queryFn: () => fetchCharacter(characterId)
  });

  // Early returns for loading/error states
  if (isLoading) return <LoadingSpinner />;
  if (error) return <ErrorBoundary error={error} />;
  if (!data) return null;

  return (
    <div className="rounded-lg border p-4">
      {/* Component content */}
    </div>
  );
}
```

## Tauri Command Pattern

**Backend (Rust):**
```rust
#[tauri::command]
async fn get_character(id: String) -> Result<Character, String> {
    // Validate input
    if id.is_empty() {
        return Err("Character ID cannot be empty".to_string());
    }

    // Implementation
    let character = fetch_character(&id).await
        .map_err(|e| e.to_string())?;

    Ok(character)
}
```

**Frontend (React):**
```tsx
// hooks/use-character.ts
import { invoke } from '@tauri-apps/api/core';
import { useQuery } from '@tanstack/react-query';

export function useCharacter(id: string) {
  return useQuery({
    queryKey: ['character', id],
    queryFn: () => invoke<Character>('get_character', { id })
  });
}
```

## Error Handling Pattern

```tsx
// Custom error boundary
<ErrorBoundary
  fallback={(error) => <ErrorMessage error={error} />}
>
  <CharacterCard {...props} />
</ErrorBoundary>
```

## File Naming

- Components: `character-card.component.tsx`
- Routes: `character-list.route.tsx`
- Styles: `character-card.styles.ts`
- Tests: `character-card.spec.tsx`
- Hooks: `use-character.ts`

## Import Organization

```tsx
// 1. External dependencies
import { z } from 'zod';
import { useQuery } from '@tanstack/react-query';

// 2. Tauri APIs
import { invoke } from '@tauri-apps/api/core';

// 3. Internal components
import { ErrorBoundary } from '../error-boundary/error-boundary.component';

// 4. Types
import type { Character } from '../../types';

// 5. Styles (if separate file)
import { cardStyles } from './character-card.styles';
```

## State Management Pattern

```tsx
// Server state: Tanstack Query
const { data } = useQuery({
  queryKey: ['characters'],
  queryFn: fetchCharacters
});

// Local UI state: useState
const [isExpanded, setIsExpanded] = useState(false);

// NO: Redux, Zustand, Context for server data
```

## QueryClient Access Pattern

**Status**: DOCUMENTED (as of 2026-01-11)

TanStack React Query's QueryClient can be accessed in two ways depending on context.

### Primary Pattern: useQueryClient Hook (Within React)

**ALWAYS use this pattern within React components and custom hooks.**

```tsx
// In component
import { useQueryClient } from '@tanstack/react-query';

export function MyComponent() {
  const queryClient = useQueryClient();

  const handleInvalidate = () => {
    queryClient.invalidateQueries({ queryKey: ['my-data'] });
  };

  return <button onClick={handleInvalidate}>Refresh</button>;
}

// In custom hook
export function useInvalidateData() {
  const queryClient = useQueryClient();

  return () => {
    queryClient.invalidateQueries({ queryKey: ['my-data'] });
  };
}

// In mutation callbacks
export function useUpdateData() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: updateData,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['data'] });
    },
  });
}
```

### Edge Case Pattern: Direct Import (Outside React)

**Only use this pattern in non-React contexts** (event handlers, utilities, Tauri command callbacks).

```tsx
// In event listener (outside React)
import { getQueryClient } from '@/queries/query-client';

// Event listener registered outside React tree
eventBus.on('backend-data-updated', (data) => {
  const queryClient = getQueryClient();
  queryClient.invalidateQueries({ queryKey: ['backend-data'] });
});
```

### When to Use Each Pattern

| Scenario | Pattern | Import |
|----------|---------|--------|
| React component | `useQueryClient()` hook | `@tanstack/react-query` |
| Custom React hook | `useQueryClient()` hook | `@tanstack/react-query` |
| Mutation callback | `useQueryClient()` hook | `@tanstack/react-query` |
| Event listener (outside React) | `getQueryClient()` | `@/queries/query-client` |
| Utility function (outside React) | `getQueryClient()` | `@/queries/query-client` |
| Test wrapper | `new QueryClient()` | `@tanstack/react-query` |

### Common Mistakes

```tsx
// WRONG - Creating new QueryClient in component
export function MyComponent() {
  const queryClient = new QueryClient(); // Creates isolated instance!
  // Use useQueryClient() instead
}

// WRONG - Using getQueryClient in component
import { getQueryClient } from '@/queries/query-client';

export function MyComponent() {
  const queryClient = getQueryClient(); // Bypasses React context!
  // Use useQueryClient() hook instead
}
```

### Architecture Decision

**Why we export the QueryClient**:
- POE2 Overlord is a Tauri SPA (not SSR) - singleton pattern is safe
- Event system may need to invalidate queries outside React context
- Follows TanStack Query recommendation for client-only apps
- Zero performance cost - just exposes existing instance

**Why we still prefer useQueryClient()**:
- React context ensures QueryClient is initialized
- Enables future testing enhancements (per-test QueryClient)
- Follows React best practices for dependency injection
- Makes component dependencies explicit

## Provider Dependency Pattern

**Status**: DOCUMENTED (as of 2026-01-11)

Application providers have specific nesting requirements. Always check `src/providers.tsx` before adding or reordering providers.

### Provider Dependency Graph

```
GameProcessProvider (independent)
  └─ ServerStatusProvider (independent)
      └─ CharacterProvider (root of character tree)
          ├─ ZoneProvider (depends on Character)
          ├─ EconomyProvider (depends on Character)
          └─ WalkthroughProvider (depends on Character)
```

### Adding a New Provider

**If provider is independent** (no dependencies):
```tsx
// Add at top level or after other independent providers
<GameProcessProvider>
  <ServerStatusProvider>
    <YourNewIndependentProvider>
      <CharacterProvider>
        {/* ... */}
      </CharacterProvider>
    </YourNewIndependentProvider>
  </ServerStatusProvider>
</GameProcessProvider>
```

**If provider depends on CharacterProvider**:
```tsx
// Must be nested INSIDE CharacterProvider
<CharacterProvider>
  <ZoneProvider>
    <EconomyProvider>
      <YourNewCharacterDependentProvider>
        <WalkthroughProvider>
          {children}
        </WalkthroughProvider>
      </YourNewCharacterDependentProvider>
    </EconomyProvider>
  </ZoneProvider>
</CharacterProvider>
```

### Rules

1. **Never reorder** without understanding dependencies
2. **Always add JSDoc** explaining new provider's dependencies
3. **Update tests** in `providers.spec.tsx` when adding providers
4. **Check context hooks** - if a provider calls `useCharacter()`, it depends on CharacterProvider

### Common Errors

```
Error: "useCharacter must be used within CharacterProvider"
Fix: Move provider inside <CharacterProvider>

Error: "useGameProcess must be used within GameProcessProvider"
Fix: Move provider inside <GameProcessProvider>
```

## Testing Pattern

**Status: CONFIGURED** (as of 2026-01-09)

Frontend testing uses Vitest + React Testing Library with co-located test files.

### Running Tests

```bash
yarn test:frontend      # Run all tests once
yarn test:watch         # Watch mode for development
yarn test:ui            # Visual test UI
yarn test:coverage      # Run with coverage report
```

### Test File Location

Tests are co-located with components using `.spec.tsx` extension:
```
src/components/ui/button/
  ├── button.tsx
  ├── button.styles.ts
  └── button.spec.tsx    # <-- test file here
```

### Basic Component Test

```tsx
// button.spec.tsx
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Button } from './button';

describe('Button', () => {
  it('renders children correctly', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByRole('button', { name: 'Click me' })).toBeInTheDocument();
  });

  it('calls onClick when clicked', async () => {
    const user = userEvent.setup();
    const handleClick = vi.fn();

    render(<Button onClick={handleClick}>Click me</Button>);
    await user.click(screen.getByRole('button'));

    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('does not call onClick when disabled', async () => {
    const user = userEvent.setup();
    const handleClick = vi.fn();

    render(<Button onClick={handleClick} disabled>Click me</Button>);
    await user.click(screen.getByRole('button'));

    expect(handleClick).not.toHaveBeenCalled();
  });
});
```

### Testing with React Query

```tsx
import { render, screen } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { describe, it, expect } from 'vitest';
import { CharacterCard } from './character-card';

describe('CharacterCard', () => {
  const createWrapper = () => {
    const queryClient = new QueryClient({
      defaultOptions: { queries: { retry: false } },
    });
    return ({ children }: { children: React.ReactNode }) => (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    );
  };

  it('renders character data', async () => {
    render(<CharacterCard characterId="123" />, { wrapper: createWrapper() });
    expect(await screen.findByText('Character Name')).toBeInTheDocument();
  });
});
```

### Key Testing Practices

1. **Use semantic queries**: Prefer `getByRole`, `getByLabelText` over `getByTestId`
2. **Test behavior, not implementation**: Focus on what users see and do
3. **Use `userEvent` for interactions**: More realistic than `fireEvent`
4. **Mock Tauri APIs**: Setup file auto-mocks `@tauri-apps/api/*`
5. **Keep tests simple**: One assertion per behavior when possible

### Mock Data Factory Pattern

Create type-safe mock factories for test data:

```tsx
const createMockCharacter = (
  overrides: Partial<CharacterData> = {}
): CharacterData => ({
  id: 'test-id',
  name: 'TestCharacter',
  class: 'Warrior',
  ascendency: 'Titan',
  level: 50,
  league: 'Standard',
  // ... all required fields with defaults
  ...overrides,
});

// Usage in tests
const character = createMockCharacter({ name: 'CustomName', level: 99 });
```

### Type-Safe Mock Functions

When mock return values include union types with null, use type assertions:

```tsx
// ❌ Incorrect - infers null type only
const mockUseCharacter = vi.hoisted(() =>
  vi.fn(() => ({
    activeCharacter: null,  // Type: null
    isLoading: false,
  }))
);

// ✅ Correct - preserves union type
const mockUseCharacter = vi.hoisted(() =>
  vi.fn(() => ({
    activeCharacter: null as CharacterData | null,  // Type: CharacterData | null
    isLoading: false,
  }))
);
```

## TypeScript Patterns

### Component Declaration Pattern

Prefer explicit function declarations over React.FC:

```tsx
// ❌ Avoid - implicit return type, no children control
export const Component: React.FC<Props> = ({ prop }) => { ... };

// ✅ Prefer - explicit return type, clear interface
export function Component({ prop }: Props): React.JSX.Element { ... }
```

### Type-Only Imports (verbatimModuleSyntax)

Always use `import type` for type-only imports:

```tsx
// ❌ Incorrect
import { ReactNode } from 'react';
import { CharacterData, Zone } from '../types';

// ✅ Correct
import type { ReactNode } from 'react';
import type { CharacterData, Zone } from '../types';

// Mixed imports - separate runtime and type imports
import { useState } from 'react';
import type { ReactNode } from 'react';
```

### Nullable Return Types

For components that may return null, explicitly declare it:

```tsx
export function ConditionalComponent({ data }: Props): React.JSX.Element | null {
  if (!data) return null;
  return <div>{data.name}</div>;
}
```
