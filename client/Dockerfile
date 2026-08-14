FROM node:24 AS build
WORKDIR /app
COPY client/package*.json ./
# npm ci needs the committed lockfile, which .gitignore used to exclude
RUN npm ci --omit=dev
COPY mountains.proto ./
COPY client/src/ ./src/

FROM gcr.io/distroless/nodejs24-debian12
WORKDIR /app
COPY --from=build /app ./
USER 1000
EXPOSE 3000
CMD ["src/index.js"]