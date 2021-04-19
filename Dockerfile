# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM ekidd/rust-musl-builder:1.50.0 as cargo-build

RUN cargo install mdbook --version 0.3.7
COPY --chown=rust:rust ./mdbook-scientific /book/mdbook-scientific
WORKDIR /book/mdbook-scientific
RUN cargo install --path .

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM ubuntu:18.04

# Install glibc which mdbook needs, if we don't use musl builder;
# Now we use something mentioned here, "ekidd/rust-musl-builder" seems the simplest:
# https://stackoverflow.com/questions/49098753/unable-to-run-a-docker-image-with-a-rust-executable
# https://dev.to/sergeyzenchenko/actix-web-in-docker-how-to-build-small-and-secure-images-2mjd

COPY --from=cargo-build /home/rust/.cargo/bin/mdbook /bin/mdbook
COPY --from=cargo-build /home/rust/.cargo/bin/mdbook-scientific /bin/mdbook-scientific

# Install latex, tikz and so on via texlive
# https://linuxconfig.org/how-to-install-latex-on-ubuntu-18-04-bionic-beaver-linux
# https://launchpad.net/ubuntu/bionic/+package/texlive-pictures
RUN apt update
ENV DEBIAN_FRONTEND=noninteractive
RUN apt install -y git gnuplot texlive texlive-latex-extra 
RUN apt install -y texlive-science texlive-pictures
# RUN apt install -y texlive-full

# Install bib2xhtml at /github/bib2xhtml/
WORKDIR /github
RUN git clone https://github.com/dspinellis/bib2xhtml.git
WORKDIR /github/bib2xhtml/
RUN ./gen-bst.pl

# Install Rust - just trying build the mdbook stuff in Ubuntu to see whether it helps, but nothing changes, commented here for future convenience
# RUN apt install -y curl
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH="${PATH}:/root/.cargo/bin"
# RUN apt install -y gcc
# RUN cargo install mdbook --version 0.3.7
# COPY --chown=rust:rust ./mdbook-scientific /book/mdbook-scientific
# WORKDIR /book/mdbook-scientific
# RUN cargo install --path .
# RUN rm /bin/mdbook*


WORKDIR /github/workspace/
ENTRYPOINT ["mdbook"]

# docker run --rm -v $(pwd):/github/workspace liufuyang/mdbook-scientific:0.3.7 build
# docker run --rm -v $(pwd):/github/workspace --name mdbook -p 3000:3000 liufuyang/mdbook-scientific:0.3.7 serve -n 0.0.0.0