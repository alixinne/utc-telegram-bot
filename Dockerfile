FROM docker.io/library/rust:buster
RUN DEBIAN_FRONTEND=noninteractive apt-get update -q && \
	apt-get install -y libsqlite3-dev
