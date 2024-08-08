# Étape 1 : Build du frontend
FROM node:18.19.1-alpine AS build-frontend

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
FROM rust:1.80.0 AS build-rust

# Définir le répertoire de travail
WORKDIR /app/api

# Créer un volume pour les dépendances Rust
VOLUME /app/api/target

# Copier les fichiers Cargo.toml et Cargo.lock
COPY api/ ./

# Installer les dépendances
RUN cargo build --release

# Étape 3 : Conteneur final
FROM debian:buster-slim

# Copier le frontend dans le conteneur final
COPY --from=build-frontend /app/frontend/dist /var/www/html

# Copier l'exécutable Rust dans le conteneur final
COPY --from=build-rust /app/api/target/release/api /usr/local/bin/api

# Exposer le port sur lequel l'application va tourner
EXPOSE 8080

# Commande pour lancer l'application
CMD 'cargo run -- "$@"'
