-- Add up migration script here
CREATE TABLE notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL CHECK (length(title) > 0),
    content TEXT NOT NULL,
    created_at TIMESTAMPZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_notes_created_at ON notes(created_at);