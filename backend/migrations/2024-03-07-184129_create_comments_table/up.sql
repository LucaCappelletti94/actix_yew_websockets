-- Extremely simple SQL defining a table for comments
CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id) ON
  DELETE
    CASCADE,
    body TEXT NOT NULL
);

-- Create a trigger function to notify a channel when a new comment is inserted
CREATE
OR REPLACE FUNCTION notify_comment() RETURNS TRIGGER AS $$
DECLARE
  channel_name TEXT;

id INTEGER;

user_id INTEGER;

body TEXT;

BEGIN
  IF TG_OP = 'INSERT'
  OR TG_OP = 'UPDATE' THEN id = NEW .id;

user_id = NEW .user_id;

body = NEW .body;

ELSE id = OLD .id;

user_id = OLD .user_id;

body = OLD .body;

END IF;

PERFORM pg_notify(
  CONCAT('comments_', NEW .user_id :: text),
  json_build_object(
    'id',
    id,
    'user_id',
    user_id,
    'body',
    body,
    'action_type',
    TG_OP
  ) :: text
);

PERFORM pg_notify(
  'comments',
  json_build_object(
    'id',
    id,
    'user_id',
    user_id,
    'body',
    body,
    'action_type',
    TG_OP
  ) :: text
);

-- Notify a channel named 'comment_added_user_<user_id>' with the user_id
RETURN NEW;

END;

$$ LANGUAGE plpgsql;

-- Add UPDATE row trigger
CREATE TRIGGER comments_update AFTER UPDATE ON comments FOR EACH ROW EXECUTE PROCEDURE notify_comment();

-- Add INSERT row trigger
CREATE TRIGGER comments_insert AFTER INSERT ON comments FOR EACH ROW EXECUTE PROCEDURE notify_comment();

-- Add DELETE row trigger
CREATE TRIGGER comments_delete AFTER DELETE ON comments FOR EACH ROW EXECUTE PROCEDURE notify_comment();
