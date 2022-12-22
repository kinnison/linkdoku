
-- Identities are the mechanism by which people can log into linkdoku

CREATE TABLE identity (
    uuid VARCHAR PRIMARY KEY,
    oidc_handle VARCHAR NOT NULL UNIQUE,
    display_name VARCHAR NOT NULL,
    gravatar_hash VARCHAR NOT NULL
);

CREATE TABLE role (
    uuid VARCHAR PRIMARY KEY,
    owner VARCHAR NOT NULL REFERENCES identity (uuid),
    display_name VARCHAR NOT NULL,
    description TEXT NOT NULL
);

CREATE INDEX role_by_owner ON role(owner);
