/**
 * QueryClient accessor utilities
 *
 * IMPORTANT: Only use these in non-React contexts (event handlers, utilities).
 * Within React components/hooks, ALWAYS use the useQueryClient() hook.
 *
 * @see packages/frontend/src/queries/README.md for usage patterns
 * @see .ai/memory/patterns.md for "QueryClient Access Pattern"
 */

import type { QueryClient } from '@tanstack/react-query';

import { queryClient } from '../main';

/**
 * Get QueryClient instance for use outside React context.
 *
 * @example
 * // In an event listener (outside React)
 * import { getQueryClient } from '@/queries/query-client';
 *
 * eventBus.on('data-changed', () => {
 *   const client = getQueryClient();
 *   client.invalidateQueries({ queryKey: ['my-data'] });
 * });
 *
 * @throws {Error} If called before QueryClient is initialized
 * @returns The singleton QueryClient instance
 */
export function getQueryClient(): QueryClient {
  if (!queryClient) {
    throw new Error('QueryClient not initialized. Ensure this is called after app startup.');
  }
  return queryClient;
}

// Re-export for direct access if needed
export { queryClient };
