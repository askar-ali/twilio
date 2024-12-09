CREATE DATABASE file_share;
CREATE USER file_share WITH ENCRYPTED PASSWORD 'file_share';
ALTER DATABASE file_share OWNER TO file_share;
GRANT ALL PRIVILEGES ON DATABASE file_share TO file_share;
GRANT USAGE,CREATE ON SCHEMA public to file_share;
ALTER DATABASE file_share SET TIMEZONE to 'Asia/Calcutta';
ALTER USER file_share CREATEDB CREATEROLE LOGIN;

/*
To open in postgres database
psql -U postgres -h localhost
*/