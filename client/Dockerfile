FROM node:22 AS build
WORKDIR /usr/src/app
COPY mountains.proto ./
COPY client/package*.json ./
COPY client/src/ ./
RUN ["npm", "install", "--omit=dev"]

FROM gcr.io/distroless/nodejs22-debian12
WORKDIR /app
COPY --from=build /usr/src/app ./
USER 1000
EXPOSE 3000
CMD ["index.js"]
