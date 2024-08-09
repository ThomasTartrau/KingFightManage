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

# Install sccache
RUN cargo install sccache

# On défini les variables d'environnement
ENV RUSTC_WRAPPER={{environment.RUSTC_WRAPPER}}
ENV API_URL={{environment.API_URL}}
ENV BISCUIT_PRIVATE_KEY={{environment.BISCUIT_PRIVATE_KEY}}
ENV DATABASE_URL={{environment.DATABASE_URL}}
ENV EMAIL_SENDER_ADDRESS={{environment.EMAIL_SENDER_ADDRESS}}
ENV EMAIL_SENDER_ADDRESS={{environment.EMAIL_SENDER_ADDRESS}}
ENV SMTP_CONNECTION_URL={{environment.SMTP_CONNECTION_URL}}
ENV WEBAPP_PATH={{environment.WEBAPP_PATH}}

# On défini le répertoire de travail pour le stage de build de l'api
WORKDIR /app/api

# Copier les fichiers de configuration de l'api
COPY api/ ./
COPY .env ./.env

# Installation des dépendances & compilation de l'api
RUN cargo build --release

## Étape 3 : stage de build final
FROM ubuntu:24.04

# Copier le frontend buildé dans le conteneur final
COPY --from=build-frontend /app/frontend/dist /prod/frontend/dist
COPY --from=build-frontend /app/frontend/public /prod/frontend/public

# Copier l'exécutable Rust dans le conteneur final
COPY --from=build-rust /app/api/target/release/api /prod/api/target/release/api
COPY --from=build-rust /app/api/.env /prod/api/.env

# Commande pour lancer l'api
CMD /prod/api/target/release/api --api-url ${API_URL} --biscuit-private-key ${BISCUIT_PRIVATE_KEY} --database-url ${DATABASE_URL} --email-sender-address ${EMAIL_SENDER_ADDRESS} --smtp-connection-url ${SMTP_CONNECTION_URL} --webapp-path ${WEBAPP_PATH}
