# Description: Dockerfile pour déployer automatiquement l'api et le frontend en production avec un Dockerfile.
# Dans notre cas on utilise un conteneur multi-stage pour builder le frontend et l'api séparément.
# Pour le provider de deployment on utilise la solution open-source Coolify sur une de nos machines.

## Étape 1 : stage de build du frontend
# Utiliser une image Node.js pour builder le frontend (on utilise la version 18.19.1-alpine car c'est la latest version & alpine pour une image plus légère)
FROM node:18.19.1-alpine AS build-frontend

# On défini le répertoire de travail pour le stage de build du frontend
WORKDIR /app/frontend

# On copie à la racine du répertoire de travail les fichiers package.json et pnpm-lock.yaml
COPY frontend/package.json frontend/pnpm-lock.yaml ./

# Installation de pnpm si ce n'est pas déjà le cas
RUN npm install -g pnpm

# Installation des dépendances
RUN pnpm install

# Copier le reste des fichiers du frontend
COPY frontend/ .

# Builder le frontend
RUN pnpm run build


# #Étape 2 : stage de build de l'api
# Utiliser une image Rust pour builder l'api (on utilise la version 1.80.0 car c'est la latest stable version)
FROM rust:1.80.0 AS build-rust

# On défini le répertoire de travail pour le stage de build de l'api
WORKDIR /app/api

# On créer un volume docker pour le répertoire de travail pour pouvoir au prochain build utiliser les dépendances déjà installées et compiler. Cela permet de gagner du temps durant le build.
VOLUME /app/storage

# Copier les fichiers de configuration de l'api
COPY api/ ./
COPY .env ./api/.env

# Installation des dépendances & compilation de l'api
RUN cargo build --release

## Étape 3 : stage de build final
FROM debian:buster-slim

# Copier le frontend buildé dans le conteneur final
COPY --from=build-frontend /app/frontend/dist /prod/frontend/dist
COPY --from=build-frontend /app/frontend/public /prod/frontend/public

# Copier l'exécutable Rust dans le conteneur final
COPY --from=build-rust /app/api/target/release/api /prod/api/target/release/api
COPY --from=build-rust /app/api/.env /prod/api/.env

# Exposer le port sur lequel l'application va tourner
EXPOSE 8080

# Commande pour lancer l'api
CMD ["/prod/api/target/release/api", "-- ", "/prod/api/.env"]
