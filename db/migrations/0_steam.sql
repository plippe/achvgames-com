CREATE TABLE IF NOT EXISTS steam_games
( id INTEGER NOT NULL
, name VARCHAR NOT NULL
, upserted_at INTEGER NOT NULL
, PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS steam_game_achievements
( name VARCHAR NOT NULL
, description VARCHAR
, icon_locked_url VARCHAR NOT NULL
, icon_unlocked_url VARCHAR NOT NULL
, steam_game_id INTEGER NOT NULL
, FOREIGN KEY (steam_game_id) REFERENCES steam_games(id) ON DELETE CASCADE
, UNIQUE (steam_game_id, name)
);
