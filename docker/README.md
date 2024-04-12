# Go to the docker folder

cd ./docker

# Copy the fake env vars

cp .env.example .env

# Pull the latest images

docker compose pull

# Start the services (in detached mode)

docker compose up -d

<!-- ssh -NL 3001:localhost:6543 root@$(docker-machine ip docker.example.com) -->
