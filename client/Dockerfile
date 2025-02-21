FROM node:23 AS build
WORKDIR /app
COPY client/package*.json ./
RUN npm install --omit=dev
COPY mountains.proto ./
COPY client/src/ ./src/

FROM gcr.io/distroless/nodejs22-debian12
WORKDIR /app
COPY --from=build /app ./
USER 1000
EXPOSE 3000
CMD ["src/index.js"]