#!/bin/sh

set -e

host="db"
shift
cmd="$@"

until PGPASSWORD=optimizer psql -h "$host" -U "forestry" -d "forestryoptimizer" -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up - running migrations"
diesel migration run

>&2 echo "Migrations complete - executing command"
exec $cmd