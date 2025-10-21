#FROM ubuntu:23.10
FROM ubuntu:24.04
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  gcc-multilib libc6-dev ca-certificates
ENV PATH=$PATH:/rust/bin
