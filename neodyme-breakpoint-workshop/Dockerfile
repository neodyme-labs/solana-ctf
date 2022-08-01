FROM rust:1.56.0-bullseye

RUN apt-get update && apt-get install -y clang libudev-dev

RUN rustup component add rustfmt

RUN adduser --gecos '' --disabled-password user
USER 1000

RUN sh -c "$(curl -sSfL https://release.solana.com/v1.8.2/install)"
ENV PATH="/home/user/.local/share/solana/install/active_release/bin:${PATH}"

WORKDIR /work

USER 0
COPY . /work
RUN chown -R user:user /work
USER 1000

RUN echo 'eval $(tr "\0" "\n" < /proc/1/environ | sed -re "s@^@export @")' > /home/user/.bashrc

# cargo run --bin <contract>
CMD while :; do :; done & kill -STOP $! && wait $!
