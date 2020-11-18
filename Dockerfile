FROM rust:1.47.0-alpine3.12

WORKDIR /advent-of-code/2020

RUN apk add --update --no-cache \
  curl \
  git \
  zsh \
  && addgroup rust \
  && adduser -G rust -s /bin/zsh -D rust \
  && chown rust:rust /advent-of-code/2020

# bash \
# gcc \
# jq \
# make \
# musl-dev \
# tzdata \

# enable once WORKDIR has a valid Cargo.toml file
# RUN cargo install --path .

CMD ["/bin/zsh"]