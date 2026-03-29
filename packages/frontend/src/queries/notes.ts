import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { useErrorHandler } from '@/hooks/useErrorHandler';
import type { CreateNoteParams, NoteData, UpdateNoteParams } from '@/types/notes';
import { parseError } from '@/utils/error-handling';

export const noteQueryKeys = {
  all: ['notes'] as const,
  lists: () => [...noteQueryKeys.all, 'list'] as const,
  pinned: () => [...noteQueryKeys.all, 'pinned'] as const,
  detail: (id: string) => [...noteQueryKeys.all, 'detail', id] as const,
};

export function useNotes() {
  return useQuery({
    queryKey: noteQueryKeys.lists(),
    queryFn: async (): Promise<NoteData[]> => {
      return await invoke<NoteData[]>('get_all_notes');
    },
    staleTime: 5 * 60 * 1000,
  });
}

export function usePinnedNotes() {
  return useQuery({
    queryKey: noteQueryKeys.pinned(),
    queryFn: async (): Promise<NoteData[]> => {
      return await invoke<NoteData[]>('get_pinned_notes');
    },
    staleTime: 5 * 60 * 1000,
  });
}

export function useNote(id: string | null) {
  return useQuery({
    queryKey: id ? noteQueryKeys.detail(id) : [...noteQueryKeys.all, 'detail', null],
    queryFn: async (): Promise<NoteData | null> => {
      if (!id) return null;
      return await invoke<NoteData>('get_note', { noteId: id });
    },
    enabled: !!id,
    staleTime: 5 * 60 * 1000,
  });
}

export function useCreateNote() {
  const queryClient = useQueryClient();
  const { handleError } = useErrorHandler();

  return useMutation({
    mutationFn: async (params: CreateNoteParams): Promise<NoteData> => {
      return await invoke<NoteData>('create_note', {
        title: params.title,
        content: params.content,
        characterId: params.character_id ?? null,
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: noteQueryKeys.all });
    },
    onError: err => {
      handleError(parseError(err));
    },
  });
}

export function useUpdateNote() {
  const queryClient = useQueryClient();
  const { handleError } = useErrorHandler();

  return useMutation({
    mutationFn: async ({
      noteId,
      params,
    }: {
      noteId: string;
      params: UpdateNoteParams;
    }): Promise<NoteData> => {
      return await invoke<NoteData>('update_note', {
        noteId,
        title: params.title,
        content: params.content,
        characterId: params.character_id ?? null,
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: noteQueryKeys.all });
    },
    onError: err => {
      handleError(parseError(err));
    },
  });
}

export function useDeleteNote() {
  const queryClient = useQueryClient();
  const { handleError } = useErrorHandler();

  return useMutation({
    mutationFn: async (noteId: string): Promise<void> => {
      return await invoke('delete_note', { noteId });
    },
    onSuccess: (_, noteId) => {
      queryClient.setQueryData(noteQueryKeys.lists(), (prev: NoteData[] | undefined) =>
        prev?.filter(n => n.id !== noteId),
      );
      queryClient.setQueryData(noteQueryKeys.pinned(), (prev: NoteData[] | undefined) =>
        prev?.filter(n => n.id !== noteId),
      );
    },
    onError: err => {
      handleError(parseError(err));
    },
  });
}

export function useToggleNotePin() {
  const queryClient = useQueryClient();
  const { handleError } = useErrorHandler();

  return useMutation({
    mutationFn: async (noteId: string): Promise<NoteData> => {
      return await invoke<NoteData>('toggle_note_pin', { noteId });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: noteQueryKeys.all });
    },
    onError: err => {
      handleError(parseError(err));
    },
  });
}
