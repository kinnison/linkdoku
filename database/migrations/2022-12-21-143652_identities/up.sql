
-- Identities are the mechanism by which people can log into linkdoku

CREATE TABLE identity (
    id SERIAL PRIMARY KEY,
    oidc_handle VARCHAR NOT NULL UNIQUE,
    display_name VARCHAR NOT NULL,
    gravatar_hash VARCHAR NOT NULL
);

