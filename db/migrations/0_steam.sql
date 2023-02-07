CREATE TABLE IF NOT EXISTS steam_games
( id BIGINT NOT NULL
, name VARCHAR NOT NULL
, upserted_at BIGINT NOT NULL
, PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS steam_game_achievements
( name VARCHAR NOT NULL
, description VARCHAR
, icon_locked_url VARCHAR NOT NULL
, icon_unlocked_url VARCHAR NOT NULL
, steam_game_id BIGINT NOT NULL
, FOREIGN KEY (steam_game_id) REFERENCES steam_games(id) ON DELETE CASCADE
, UNIQUE (steam_game_id, name)
);
