import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render as rtlRender } from '@testing-library/react';
import type { ReactElement, ReactNode } from 'react';
import { Providers } from '../providers';

/**
 * Custom render function that wraps components with all application providers.
 *
 * ⚠️ WHEN TO USE THIS:
 * - Integration tests that need real provider behavior
 * - Testing components that rely on multiple context interactions
 * - Testing cross-context effects (e.g., updating a character affects zones)
 * - Route-level tests that need the full app context
 *
 * ⚠️ WHEN NOT TO USE (use @testing-library/react instead):
 * - Unit tests of presentational components (buttons, cards, forms)
 * - Tests with mocked contexts (vi.mock) - mocks are faster than real providers
 * - Tests where provider overhead would slow down the suite
 * - ANY test checking static rendering (does element X appear?)
 *
 * 📊 PERFORMANCE IMPACT:
 * This wrapper creates the full provider tree (QueryClient + 6 app contexts) for EVERY render.
 * - Plain RTL render: ~5-10ms per test
 * - This custom render: ~30-50ms per test
 *
 * Most tests should use plain RTL render + mocks for speed. This is here for when you
 * genuinely need integrated provider behavior, not as a default.
 *
 * @example
 * // Integration test - USE this custom render
 * import { render, screen } from '@/test/test-utils';
 * render(<CharacterListPage />);
 *
 * @example
 * // Unit test - DON'T use this, use plain RTL
 * import { render, screen } from '@testing-library/react';
 * vi.mock('@/hooks/useCharacter', () => ({ ... }));
 * render(<CharacterCard character={mockChar} />);
 *
 * @example
 * // With custom query client for testing React Query behavior
 * import { render } from '@/test/test-utils';
 * const queryClient = new QueryClient({ ... });
 * render(<MyComponent />, { queryClient });
 */
export function render(
  ui: ReactElement,
  options?: {
    queryClient?: QueryClient;
  },
) {
  const queryClient =
    options?.queryClient ||
    new QueryClient({
      defaultOptions: {
        queries: {
          retry: false,
          gcTime: 0,
        },
      },
    });

  function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>
        <Providers>{children}</Providers>
      </QueryClientProvider>
    );
  }

  return rtlRender(ui, { wrapper: Wrapper });
}

// Re-export everything from RTL
export * from '@testing-library/react';
export { default as userEvent } from '@testing-library/user-event';
