\set pguser `echo "$POSTGRES_USER"`
\set pgpass `echo "$POSTGRES_PASSWORD"`

ALTER USER :pguser WITH PASSWORD :'pgpass';
