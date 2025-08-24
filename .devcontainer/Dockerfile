# Base image
FROM rust:1.89.0
# FROM mcr.microsoft.com/devcontainers/rust:1

# Run
RUN apt-get update && apt-get install -y \
    git \
    tree \
    nano \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Where to work
WORKDIR /workspace
# Needed to get $WORKDIR
ENV WORKDIR=/workspace

# Dealing with sub-projects: p_*
# Copying everything in /tmp/context then only the sub-projects in $WORKDIR.
# Not possible with the COPY instruction.
COPY . /tmp/context/
RUN ls -la /tmp/context > $WORKDIR/context.txt
RUN cp -r /tmp/context/p_* $WORKDIR
RUN rm -fr /tmp/context

#
COPY .devcontainer/ .devcontainer/
COPY .github/ .github/
COPY .gitattributes .
COPY .gitignore .
COPY Cargo.toml .
COPY LICENSE .
COPY README.md .
COPY run_me.sh .


# Rust stuff
RUN rustup component add rustfmt clippy
