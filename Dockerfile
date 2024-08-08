# Étape 1 : Build du frontend
FROM node:18.19.1 AS build-frontend

# Définir le répertoire de travail
WORKDIR /app/frontend

# Copier les fichiers package.json et pnpm-lock.yaml
COPY frontend/package.json frontend/pnpm-lock.yaml ./

# Installer pnpm
RUN npm install -g pnpm

# Installer les dépendances et builder le frontend
RUN pnpm install
COPY frontend/ .
RUN pnpm run build

# Étape 2 : Builder et lancer l'application Rust
FROM rust:1.19.0 AS build-rust

# Définir le répertoire de travail
WORKDIR /app

# Copier les fichiers source Rust
COPY api/Cargo.toml api/Cargo.lock ./
COPY api/src ./src

# Compiler l'application Rust en mode release
RUN cargo build --release

# Étape 3 : Conteneur final
FROM debian:buster-slim

# Copier le frontend dans le conteneur final
COPY --from=build-frontend /app/frontend/dist /var/www/html

# Copier l'exécutable Rust dans le conteneur final
COPY --from=build-rust /app/target/release/api /usr/local/bin/api

# Exposer le port sur lequel l'application va tourner
EXPOSE 8080

# Commande pour lancer l'application
CMD 'cargo run -- "$@"'
