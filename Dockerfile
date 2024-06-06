FROM rust:1.77.2-slim

RUN apt-get update && \
    apt-get install -y curl libxml2-utils perl openssh-client && \
    apt-get clean

WORKDIR /usr/ceremony
ADD . .

ENV RUST_LOG=info
RUN cargo install --locked --path . --root .
ENV PATH="/usr/ceremony/bin:${PATH}"

CMD ["bash"]