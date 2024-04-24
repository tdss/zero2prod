CREATE TABLE game(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  reward TEXT,
  monetary_reward integer,
  monetary_reward_increase integer,
  created_at timestamptz,
  launched_at timestamptz NOT NULL,
  time_reset_period integer NOT NULL,
  max_number_of_players integer NOT NULL
);

CREATE TABLE player(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  email TEXT
);

CREATE TABLE players_to_games(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  plyer_id uuid NOT NULL,
  game_id uuid NOT NULL
);

CREATE TABLE game_history(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  player_id uuid NOT NULL,
  game_id uuid NOT NULL,
  timestamp timestamptz NOT NULL,
  action varchar(32) NOT NULL,
  description TEXT
)
