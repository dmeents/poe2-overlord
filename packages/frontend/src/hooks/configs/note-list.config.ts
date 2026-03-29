import type { NoteData } from '../../types/notes';

export interface NoteFilters {
  search: string;
  pinned: boolean | null;
  character_id: string | null;
}

export type NoteSortField = 'updated_at' | 'created_at' | 'title';

export const noteListConfig = {
  defaultFilters: {
    search: '',
    pinned: null,
    character_id: null,
  } as NoteFilters,

  defaultSort: {
    field: 'updated_at' as NoteSortField,
    direction: 'desc' as const,
  },

  filterFn: (note: NoteData, filters: NoteFilters): boolean => {
    if (filters.pinned !== null && note.is_pinned !== filters.pinned) {
      return false;
    }

    if (filters.character_id !== null && note.character_id !== filters.character_id) {
      return false;
    }

    if (filters.search.trim() !== '') {
      const term = filters.search.toLowerCase().trim();
      if (!note.title.toLowerCase().includes(term) && !note.content.toLowerCase().includes(term)) {
        return false;
      }
    }

    return true;
  },

  sortFn: (
    a: NoteData,
    b: NoteData,
    sort: { field: NoteSortField; direction: 'asc' | 'desc' },
  ): number => {
    let comparison = 0;

    switch (sort.field) {
      case 'updated_at': {
        comparison = new Date(a.updated_at).getTime() - new Date(b.updated_at).getTime();
        break;
      }
      case 'created_at': {
        comparison = new Date(a.created_at).getTime() - new Date(b.created_at).getTime();
        break;
      }
      case 'title': {
        comparison = a.title.localeCompare(b.title);
        break;
      }
      default: {
        comparison = 0;
      }
    }

    return sort.direction === 'asc' ? comparison : -comparison;
  },

  chipConfigs: [
    {
      key: 'pinned' as const,
      label: (value: boolean | null) => (value ? 'Pinned only' : 'Unpinned only'),
      isActive: (value: boolean | null) => value !== null,
    },
    {
      key: 'search' as const,
      label: (value: string) => `Search: ${value}`,
      isActive: (value: string) => value.trim() !== '',
    },
    {
      key: 'character_id' as const,
      label: (_value: string | null) => 'Character filter',
      isActive: (value: string | null) => value !== null,
    },
  ],
};
