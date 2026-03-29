export interface NoteData {
  id: string;
  title: string;
  content: string;
  is_pinned: boolean;
  character_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateNoteParams {
  title: string;
  content: string;
  character_id?: string | null;
}

export interface UpdateNoteParams {
  title: string;
  content: string;
  character_id?: string | null;
}
