
-- Identities are the mechanism by which people can log into linkdoku

CREATE TABLE identity (
    id SERIAL PRIMARY KEY,
    oidc_handle VARCHAR NOT NULL UNIQUE,
    display_name VARCHAR NOT NULL,
    gravatar_hash VARCHAR NOT NULL
);

CREATE TABLE role (
    id SERIAL PRIMARY KEY,
    owner INTEGER NOT NULL REFERENCES identity (id),
    display_name VARCHAR NOT NULL,
    description TEXT NOT NULL
);

CREATE INDEX role_by_owner ON role(owner);
