FROM rust:slim as build

# this is needed for the openssl issues during image build
RUN apt-get update && apt-get install -y curl libssl-dev pkg-config build-essential

EXPOSE 8080

WORKDIR /app
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

WORKDIR /usr/local/bin

COPY --from=build /app/target/release/simple_restaurant_jrbangit_paidy .

CMD ["simple_restaurant_jrbangit_paidy"]