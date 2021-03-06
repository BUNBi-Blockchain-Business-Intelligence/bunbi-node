FROM phusion/baseimage:0.11 as builder
LABEL maintainer "admin@bunbi.com.mx"

WORKDIR /bunbi

COPY . /bunbi

RUN apt-get update && \
	apt-get install -y cmake pkg-config libssl-dev git clang
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        export PATH=$PATH:$HOME/.cargo/bin && \
        scripts/init.sh && \
        cargo build --release

FROM phusion/baseimage:0.11


COPY --from=builder /bunbi/target/release/bunbi-node /usr/local/bin

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	rm -rf /usr/lib/python* && \
	useradd -m -u 1000 -U -s /bin/sh -d /bunbi bunbi && \
	mkdir -p /bunbi/.local/share/bunbi-node && \
	chown -R bunbi:bunbi /bunbi/.local && \
	ln -s /bunbi/.local/share/bunbi-node /data && \
	rm -rf /usr/bin /usr/sbin

USER bunbi
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/bunbi-node"]
