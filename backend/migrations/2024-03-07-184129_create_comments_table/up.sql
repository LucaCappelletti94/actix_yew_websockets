-- Extremely simple SQL defining a table for comments
CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  body TEXT NOT NULL
);

-- Create a trigger function to notify a channel when a new comment is inserted
CREATE OR REPLACE FUNCTION notify_new_comment() RETURNS TRIGGER AS $$
BEGIN
  PERFORM pg_notify('comment_added', NEW.user_id::text); -- Notify a channel named 'comment_added' with the user_id
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the notify_new_comment function after each insert
CREATE TRIGGER comment_insert_trigger
AFTER INSERT ON comments
FOR EACH ROW
EXECUTE FUNCTION notify_new_comment();

-- Create a trigger function to notify a channel when a new comment is inserted
CREATE OR REPLACE FUNCTION notify_user_new_comment() RETURNS TRIGGER AS $$
DECLARE
  channel_name TEXT;
BEGIN
  channel_name := CONCAT('comment_added_user_', NEW.user_id::text);
  PERFORM pg_notify(channel_name, NEW.user_id::text); -- Notify a channel named 'comment_added_user_<user_id>' with the user_id
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the notify_new_comment function after each insert
CREATE TRIGGER comment_user_insert_trigger
AFTER INSERT ON comments
FOR EACH ROW
EXECUTE FUNCTION notify_user_new_comment();
