FROM ubuntu

RUN apt update && apt upgrade -y && \
    apt install git wget curl tar -y && \
    mkdir /home/kwik

WORKDIR /home/kwik
COPY release/kwik /usr/local/bin/
CMD ["/bin/sh", "-l"]
