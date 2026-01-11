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
