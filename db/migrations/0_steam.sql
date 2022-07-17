CREATE TABLE IF NOT EXISTS steam_games
( id INTEGER NOT NULL
, name VARCHAR NOT NULL
, created_at timestamp NOT NULL DEFAULT current_timestamp
, updated_at timestamp NOT NULL DEFAULT current_timestamp
, PRIMARY KEY (id)
);

CREATE TRIGGER steam_games_update_updated_at
AFTER UPDATE ON steam_games
BEGIN
    UPDATE steam_games
    SET updated_at = current_timestamp
    WHERE id = NEW.id;
END;

CREATE TABLE IF NOT EXISTS steam_game_achievements
( name VARCHAR NOT NULL
, description VARCHAR
, icon_locked_url VARCHAR NOT NULL
, icon_unlocked_url VARCHAR NOT NULL
, steam_game_id INTEGER NOT NULL
, FOREIGN KEY (steam_game_id) REFERENCES steam_games(id) ON DELETE CASCADE
, UNIQUE (steam_game_id, name)
);
