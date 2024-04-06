FROM mysql:latest
ENV MYSQL_DATABASE=app_pixel-db-1 \
    MYSQL_ROOT_PASSWORD=password
COPY ./create_table.sql /docker-entrypoint-initdb.d/
RUN cargo build --release
