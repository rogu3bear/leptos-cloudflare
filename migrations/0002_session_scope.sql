ALTER TABLE todos ADD COLUMN session_id TEXT NOT NULL DEFAULT '';

UPDATE todos
SET session_id = 'legacy'
WHERE session_id = '';

CREATE INDEX IF NOT EXISTS idx_todos_session_completed_id
ON todos (session_id, completed, id DESC);
