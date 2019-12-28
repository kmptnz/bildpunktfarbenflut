FROM rust:latest

# Build-time metadata as defined at http://label-schema.org

LABEL org.label-schema.build-date=$BUILD_DATE \
      org.label-schema.name="pixelpwnr docker" \
      org.label-schema.description="A quick pixelflut (video) client in Rust for use at 34C3, that pwns whole pixelflut panels" \
      org.label-schema.url="https://gitlab.com/kmptnz" \
      org.label-schema.vendor="kmpt.nz" \
      org.label-schema.version=$CI_COMMIT_REF_SLUG \
      org.label-schema.schema-version="1.0"


WORKDIR /usr/src/pixelpwnr

COPY pixelpwnr .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/pixelpwnr"]