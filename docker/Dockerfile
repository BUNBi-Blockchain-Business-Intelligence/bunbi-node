FROM phusion/baseimage:0.11 as builder
LABEL maintainer "admin@bunbi.com.mx"

WORKDIR /bunbisocial

COPY . /bunbisocial

RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install -y cmake pkg-config libssl-dev git clang
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        export PATH=$PATH:$HOME/.cargo/bin && \
        scripts/init.sh && \
        cargo build --release

FROM phusion/baseimage:0.11


COPY --from=builder /bunbisocial/target/release/bunbi-node /usr/local/bin

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	rm -rf /usr/lib/python* && \
	useradd -m -u 1000 -U -s /bin/sh -d /bunbisocial bunbisocial && \
	mkdir -p /bunbisocial/.local/share/bunbi-node && \
	chown -R bunbisocial:bunbisocial /bunbisocial/.local && \
	ln -s /bunbisocial/.local/share/bunbi-node /data && \
	rm -rf /usr/bin /usr/sbin

USER bunbisocial
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/bunbi-node"]
