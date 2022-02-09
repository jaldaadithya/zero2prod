-- Add migration script here
CREATE TABLE subscriptions(
   id bigint(20) NOT NULL AUTO_INCREMENT,
   PRIMARY KEY (id),
   email VARCHAR(100) NOT NULL UNIQUE,
   name VARCHAR(100) NOT NULL,
   subscribed_at timestamp NOT NULL
);