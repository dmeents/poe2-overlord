import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { createRouter, RouterProvider } from '@tanstack/react-router';
import { StrictMode } from 'react';
import ReactDOM from 'react-dom/client';
import { Providers } from './providers';

// Import the generated route tree
import { routeTree } from './routeTree.gen';

// Create a new router instance
const router = createRouter({ routeTree });

/**
 * Global QueryClient instance.
 *
 * IMPORTANT: Within React components/hooks, always use the useQueryClient() hook
 * instead of importing this directly. Direct import is only for edge cases
 * outside React context (event handlers, utilities).
 *
 * @see packages/frontend/src/queries/README.md for usage patterns
 * @see .ai/memory/patterns.md for "QueryClient Access Pattern"
 */
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 minutes
      gcTime: 10 * 60 * 1000, // 10 minutes (formerly cacheTime)
    },
  },
});

// Register the router instance for type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router;
  }
}

// Render the app
const rootElement = document.getElementById('root')!;

if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);

  root.render(
    <StrictMode>
      <QueryClientProvider client={queryClient}>
        <Providers>
          <RouterProvider router={router} />
        </Providers>
      </QueryClientProvider>
    </StrictMode>
  );
}
