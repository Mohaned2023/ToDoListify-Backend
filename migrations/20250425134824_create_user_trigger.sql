-- Add migration script here
CREATE OR REPLACE FUNCTION set_user_inactive()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE users
    SET state = 'inactive'
    WHERE id = OLD.id;

    DELETE FROM sessions
    WHERE user_id = OLD.id;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_delete_user
BEFORE DELETE ON users
FOR EACH ROW
EXECUTE FUNCTION set_user_inactive();
