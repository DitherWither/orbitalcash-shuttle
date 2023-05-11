dev: copy-all
    cargo shuttle run --port 3032 &
    npm --prefix ./frontend/ run dev -- --port 3031 &
    caddy run
    npx kill-port 3031

copy-all:
    cp -u Secrets.toml api/shuttle-rocket/
    cp -u Secrets.dev.toml api/shuttle-rocket/
    cp -ur dist api/shuttle-rocket/

build-frontend:
    npm --prefix ./frontend run build

deploy: build-frontend copy-all
    cargo shuttle deploy

create-dev-db:
    docker run --name postgres -e POSTGRES_PASSWORD=password -d postgres

dev-kill-ports:
    npx kill-port 3032
    npx kill-port 3031
    npx kill-port 3030