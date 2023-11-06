docker rm $(docker ps -a -q)
docker volume rm postgresql_data
