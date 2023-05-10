dev:
    cargo shuttle run --port 3032 &
    npm --prefix ./frontend/ run dev -- --port 3031 &
    caddy run

build-frontend:
    npm --prefix ./frontend run build

deploy: build-frontend
    cargo shuttle deploy

dev-kill-ports:
    npx kill-port 3032
    npx kill-port 3031
    npx kill-port 3030