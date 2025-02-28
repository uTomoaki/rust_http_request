FROM node:16

RUN npm install -g json-server

WORKDIR /app

COPY ./mock/db.json /app/db.json

CMD ["json-server", "--watch", "db.json", "--port", "4010"]
