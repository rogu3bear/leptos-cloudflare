CREATE TABLE anonymous_admission_key (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  secret TEXT NOT NULL CHECK (length(secret) = 64)
);

ALTER TABLE todos ADD COLUMN admission_key TEXT NOT NULL DEFAULT '';
ALTER TABLE contact_submissions ADD COLUMN admission_key TEXT NOT NULL DEFAULT '';
CREATE INDEX idx_todos_admission ON todos (admission_key);
CREATE INDEX idx_contact_admission_created ON contact_submissions (admission_key, created_at);
