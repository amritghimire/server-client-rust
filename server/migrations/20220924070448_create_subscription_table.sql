-- Create Subscription table
CREATE TABLE subscription(
     id uuid NOT NULL,
     PRIMARY KEY (id),
     email TEXT NOT NULL unique ,
     name TEXT NOT NULL ,
     subscribed_at timestamptz NOT NULL
);