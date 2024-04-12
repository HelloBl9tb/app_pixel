\set pguser `echo "$POSTGRES_USER"`

-- creat database "pix" if not exists
SELECT 'CREATE DATABASE pix'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'pix')\gexec
grant all privileges on database pix to :pguser;
