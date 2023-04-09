---- Base app schema
---- - Timestamps 
----   - cid/ctime for the creator id and time. 
----   - mid/mtime for the last modifier id and time


-- User
CREATE TABLE "user" (
  id bigint GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  username varchar(128) NOT NULL UNIQUE,

  -- Timestamps
  cid bigint NOT NULL,
  ctime timestamp with time zone NOT NULL,
  mid bigint NOT NULL,
  mtime timestamp with time zone NOT NULL,

  -- Auth
  pwd varchar(256),
  pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
  token_salt uuid NOT NULL DEFAULT gen_random_uuid()
);


-- Ticket
CREATE TABLE ticket (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  -- Timestamps
  cid bigint NOT NULL, 
  ctime timestamp with time zone NOT NULL,
  mid bigint NOT NULL,
  mtime timestamp with time zone NOT NULL,  

  title varchar(256)
);