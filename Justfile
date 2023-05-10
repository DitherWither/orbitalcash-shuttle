dev: copy-config
    cargo shuttle run --port 3032 &
    npm --prefix ./frontend/ run dev -- --port 3031 &
    caddy run
    npx kill-port 3031

copy-config:
    cp -u Secrets.toml api/shuttle-rocket/
    cp -u Secrets.dev.toml api/shuttle-rocket/

build-frontend:
    npm --prefix ./frontend run build

deploy: build-frontend copy-config
    cargo shuttle deploy

dev-kill-ports:
    npx kill-port 3032
    npx kill-port 3031
    npx kill-port 3030