FROM node:16-alpine AS build
WORKDIR /repo
COPY pkg pkg
COPY www www
WORKDIR /repo/www
RUN npm ci && npm run build

FROM nginx:alpine
COPY --from=build /repo/www/dist /usr/share/nginx/html
