`docker run --rm   --name postgres -e POSTGRES_USER=astriddegaste -e POSTGRES_PASSWORD=password -d -p 5432:5432 -v $HOME/docker/volumes/postgres:/var/lib/postgresql/data  postgres`

https://hackernoon.com/dont-install-postgres-docker-pull-postgres-bee20e200198

`psql -h localhost -U astriddegaste -d postgres`
then `password`
