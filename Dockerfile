# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM ekidd/rust-musl-builder:1.50.0 as cargo-build

RUN cargo install mdbook --version 0.3.7
COPY --chown=rust:rust . /book
WORKDIR /book/mdbook-scientific
RUN cargo install --path .

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine/git:v2.30.1

# Install glibc which mdbook needs, if we don't use musl builder;
# Now we use something mentioned here, "ekidd/rust-musl-builder" seems the simplest:
# https://stackoverflow.com/questions/49098753/unable-to-run-a-docker-image-with-a-rust-executable
# https://dev.to/sergeyzenchenko/actix-web-in-docker-how-to-build-small-and-secure-images-2mjd

COPY --from=cargo-build /home/rust/.cargo/bin/mdbook /bin/mdbook
COPY --from=cargo-build /home/rust/.cargo/bin/mdbook-scientific /bin/mdbook-scientific

WORKDIR /github/workspace/
ENTRYPOINT ["mdbook"]

# docker run --rm -it -v $(pwd):/github/workspace liufuyang/mdbook-scientific:0.4.7 build