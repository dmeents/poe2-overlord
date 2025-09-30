import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useCallback, useEffect, useRef, useState } from 'react';

/**
 * Generic CRUD operation types
 * @template T - The type of the entity
 * @template CreateData - The type of data needed to create an entity
 * @template UpdateData - The type of data needed to update an entity
 * @template ListParams - The type of parameters for listing entities
 */
export interface CRUDOperations<T extends { id: string }, CreateData, UpdateData, ListParams = void> {
  /** Get all entities */
  getAll: (params?: ListParams) => Promise<T[]>;
  /** Get a specific entity by ID */
  getById: (id: string) => Promise<T | null>;
  /** Create a new entity */
  create: (data: CreateData) => Promise<T>;
  /** Update an existing entity */
  update: (id: string, data: UpdateData) => Promise<T>;
  /** Delete an entity */
  delete: (id: string) => Promise<void>;
  /** Set active entity (if applicable) */
  setActive?: (id: string) => Promise<void>;
}

/**
 * Configuration for CRUD operations hook
 * @template T - The type of the entity
 * @template CreateData - The type of data needed to create an entity
 * @template UpdateData - The type of data needed to update an entity
 * @template ListParams - The type of parameters for listing entities
 */
export interface CRUDOperationsConfig<T extends { id: string }, CreateData, UpdateData, ListParams = void> {
  /** The CRUD operations implementation */
  operations: CRUDOperations<T, CreateData, UpdateData, ListParams>;
  /** Query key prefix for React Query caching */
  queryKeyPrefix: string;
  /** Optional function to get active entity */
  getActive?: () => Promise<T | null>;
  /** Optional function to set active entity */
  setActive?: (id: string) => Promise<void>;
  /** Stale time for queries in milliseconds (default: 5 minutes) */
  staleTime?: number;
  /** Whether to enable real-time updates via events */
  enableRealTimeUpdates?: boolean;
  /** Event name for real-time updates */
  updateEventName?: string;
  /** Function to handle real-time update events */
  handleUpdateEvent?: (event: { payload: unknown }, entities: T[]) => T[];
}

/**
 * Generic hook for CRUD operations with React Query integration
 * 
 * This hook provides a reusable pattern for CRUD operations with automatic
 * caching, optimistic updates, and real-time event handling. It can replace
 * the duplicate logic in useCharacterManagement and similar hooks.
 * 
 * @template T - The type of the entity
 * @template CreateData - The type of data needed to create an entity
 * @template UpdateData - The type of data needed to update an entity
 * @template ListParams - The type of parameters for listing entities
 * @param config - Configuration object for CRUD operations
 * @returns Object containing data, loading states, and CRUD functions
 * 
 * @example
 * ```typescript
 * interface MyEntity {
 *   id: string;
 *   name: string;
 *   value: number;
 * }
 * 
 * interface CreateData {
 *   name: string;
 *   value: number;
 * }
 * 
 * const operations: CRUDOperations<MyEntity, CreateData, CreateData> = {
 *   getAll: () => invoke('get_all_entities'),
 *   getById: (id) => invoke('get_entity', { id }),
 *   create: (data) => invoke('create_entity', data),
 *   update: (id, data) => invoke('update_entity', { id, data }),
 *   delete: (id) => invoke('delete_entity', { id }),
 * };
 * 
 * const config: CRUDOperationsConfig<MyEntity, CreateData, CreateData> = {
 *   operations,
 *   queryKeyPrefix: 'entities',
 *   enableRealTimeUpdates: true,
 *   updateEventName: 'entity-updated',
 * };
 * 
 * const { entities, activeEntity, isLoading, create, update, delete: deleteEntity } = 
 *   useCRUDOperations(config);
 * ```
 */
export function useCRUDOperations<T extends { id: string }, CreateData, UpdateData, ListParams = void>(
  config: CRUDOperationsConfig<T, CreateData, UpdateData, ListParams>
) {
  const {
    operations,
    queryKeyPrefix,
    getActive,
    setActive,
    staleTime = 5 * 60 * 1000,
    enableRealTimeUpdates = false,
    updateEventName,
    handleUpdateEvent,
  } = config;

  const queryClient = useQueryClient();

  // Query keys for consistent caching
  const queryKeys = {
    all: [queryKeyPrefix] as const,
    lists: () => [...queryKeys.all, 'list'] as const,
    list: (params?: ListParams) =>
      [...queryKeys.lists(), { params }] as const,
    details: () => [...queryKeys.all, 'detail'] as const,
    detail: (id: string) => [...queryKeys.details(), id] as const,
    active: () => [...queryKeys.all, 'active'] as const,
  };

  // State for real-time updates
  const [entitiesWithUpdates, setEntitiesWithUpdates] = useState<T[]>([]);
  const [activeEntityWithUpdates, setActiveEntityWithUpdates] = useState<T | null>(null);
  const listenerRef = useRef<(() => void) | null>(null);
  const isListeningRef = useRef(false);

  // Query hooks
  const {
    data: entities = [],
    isLoading: entitiesLoading,
    error: entitiesError,
  } = useQuery({
    queryKey: queryKeys.lists(),
    queryFn: () => operations.getAll(),
    staleTime,
  });

  const {
    data: activeEntity = null,
    isLoading: activeEntityLoading,
    error: activeEntityError,
  } = useQuery({
    queryKey: queryKeys.active(),
    queryFn: () => getActive?.() ?? Promise.resolve(null),
    enabled: !!getActive,
    staleTime,
  });

  // Initialize entities with updates when data changes
  useEffect(() => {
    setEntitiesWithUpdates(entities);
  }, [entities]);

  // Initialize active entity with updates when data changes
  useEffect(() => {
    setActiveEntityWithUpdates(activeEntity);
  }, [activeEntity]);

  // Real-time update handler
  const handleRealTimeUpdate = useCallback(
    (event: { payload: unknown }) => {
      if (handleUpdateEvent) {
        setEntitiesWithUpdates(prev => handleUpdateEvent(event, prev));
        
        // Update active entity if it's affected
        if (activeEntity) {
          const updatedEntities = handleUpdateEvent(event, [activeEntity]);
          if (updatedEntities.length > 0) {
            setActiveEntityWithUpdates(updatedEntities[0]);
          }
        }
      }
    },
    [handleUpdateEvent, activeEntity]
  );

  // Set up real-time event listener
  useEffect(() => {
    if (!enableRealTimeUpdates || !updateEventName || !handleUpdateEvent) {
      return;
    }

    // Clean up existing listener
    if (listenerRef.current) {
      listenerRef.current();
      listenerRef.current = null;
    }

    // Prevent multiple listeners
    if (isListeningRef.current) {
      return;
    }

    isListeningRef.current = true;

    const setupListener = async () => {
      try {
        const { listen } = await import('@tauri-apps/api/event');
        const unlisten = await listen(updateEventName, handleRealTimeUpdate);
        listenerRef.current = unlisten;
      } catch (error) {
        console.error('Failed to set up real-time listener:', error);
        isListeningRef.current = false;
      }
    };

    setupListener();

    return () => {
      if (listenerRef.current) {
        listenerRef.current();
        listenerRef.current = null;
      }
      isListeningRef.current = false;
    };
  }, [enableRealTimeUpdates, updateEventName, handleUpdateEvent]);

  // Mutation hooks
  const createMutation = useMutation({
    mutationFn: operations.create,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.all });
    },
  });

  const updateMutation = useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateData }) =>
      operations.update(id, data),
    onSuccess: (updatedEntity) => {
      // Update the specific entity in cache
      queryClient.setQueryData(queryKeys.detail(updatedEntity.id), updatedEntity);
      // Invalidate lists to ensure consistency
      queryClient.invalidateQueries({ queryKey: queryKeys.lists() });
      if (getActive) {
        queryClient.invalidateQueries({ queryKey: queryKeys.active() });
      }
    },
  });

  const deleteMutation = useMutation({
    mutationFn: operations.delete,
    onSuccess: (_, deletedId) => {
      // Remove the entity from cache
      queryClient.removeQueries({ queryKey: queryKeys.detail(deletedId) });
      // Invalidate lists and active entity
      queryClient.invalidateQueries({ queryKey: queryKeys.lists() });
      if (getActive) {
        queryClient.invalidateQueries({ queryKey: queryKeys.active() });
      }
    },
  });

  const setActiveMutation = useMutation({
    mutationFn: operations.setActive || setActive || (() => Promise.resolve()),
    onSuccess: () => {
      if (getActive) {
        queryClient.invalidateQueries({ queryKey: queryKeys.active() });
      }
    },
  });

  // Derived state
  const isLoading = entitiesLoading || activeEntityLoading;
  const error = entitiesError?.message || activeEntityError?.message || null;

  // CRUD functions with error handling
  const create = useCallback(
    async (data: CreateData) => {
      try {
        return await createMutation.mutateAsync(data);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to create entity';
        throw new Error(errorMessage);
      }
    },
    [createMutation]
  );

  const update = useCallback(
    async (id: string, data: UpdateData) => {
      try {
        return await updateMutation.mutateAsync({ id, data });
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to update entity';
        throw new Error(errorMessage);
      }
    },
    [updateMutation]
  );

  const deleteEntity = useCallback(
    async (id: string) => {
      try {
        await deleteMutation.mutateAsync(id);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to delete entity';
        throw new Error(errorMessage);
      }
    },
    [deleteMutation]
  );

  const setActiveEntity = useCallback(
    async (id: string) => {
      if (!operations.setActive && !setActive) {
        throw new Error('Set active operation not configured');
      }
      try {
        await setActiveMutation.mutateAsync(id);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to set active entity';
        throw new Error(errorMessage);
      }
    },
    [setActiveMutation, operations.setActive, setActive]
  );

  // Legacy functions for backward compatibility
  const loadEntities = useCallback(() => {
    // No-op: React Query handles this automatically
  }, []);

  const loadActiveEntity = useCallback(() => {
    // No-op: React Query handles this automatically
  }, []);

  return {
    // Return entities with real-time updates
    entities: entitiesWithUpdates,
    // Return active entity with real-time updates
    activeEntity: activeEntityWithUpdates,
    isLoading,
    error,
    loadEntities,
    loadActiveEntity,
    create,
    update,
    delete: deleteEntity,
    setActive: setActiveEntity,
    // Mutation states for advanced usage
    createMutation,
    updateMutation,
    deleteMutation,
    setActiveMutation,
  };
}

/**
 * Helper function to create CRUD operations configuration
 * @template T - The type of the entity
 * @template CreateData - The type of data needed to create an entity
 * @template UpdateData - The type of data needed to update an entity
 * @template ListParams - The type of parameters for listing entities
 * @param operations - The CRUD operations implementation
 * @param queryKeyPrefix - Query key prefix for React Query caching
 * @param options - Optional configuration
 * @returns CRUD operations configuration object
 */
export function createCRUDOperationsConfig<T extends { id: string }, CreateData, UpdateData, ListParams = void>(
  operations: CRUDOperations<T, CreateData, UpdateData, ListParams>,
  queryKeyPrefix: string,
  options?: {
    getActive?: () => Promise<T | null>;
    setActive?: (id: string) => Promise<void>;
    staleTime?: number;
    enableRealTimeUpdates?: boolean;
    updateEventName?: string;
    handleUpdateEvent?: (event: { payload: unknown }, entities: T[]) => T[];
  }
): CRUDOperationsConfig<T, CreateData, UpdateData, ListParams> {
  return {
    operations,
    queryKeyPrefix,
    getActive: options?.getActive,
    setActive: options?.setActive,
    staleTime: options?.staleTime,
    enableRealTimeUpdates: options?.enableRealTimeUpdates,
    updateEventName: options?.updateEventName,
    handleUpdateEvent: options?.handleUpdateEvent,
  };
}
