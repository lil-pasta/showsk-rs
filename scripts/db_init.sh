#!/bin/bash 
set -x
set -eo pipefail

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
SKIP_DOCKER=false

# Allow to skip Docker if a dockerized Postgres database is already running
docker run \
	-e POSTGRES_USER=${DB_USER} \
	-e POSTGRES_PASSWORD=${DB_PASSWORD} \
	-e POSTGRES_DB=${DB_NAME} \
	-p "${DB_PORT}":5432 \
	-d postgres \
	postgres -N 1000

export PGPASSWORD="${DB_PASSWORD}"

until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
	>&2 echo "Postgres is still unavailable - sleeping" 
	sleep 1 
done
>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
cargo sqlx database create
cargo sqlx migrate run

>&2 echo "migrated and ready to party"
