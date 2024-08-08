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
WORKDIR /app/api

# Copier les fichiers Cargo.toml et Cargo.lock
COPY api/Cargo.toml api/Cargo.lock ./

# Installer les dépendances
RUN cargo build --release