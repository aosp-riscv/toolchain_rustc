FROM ubuntu:16.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  g++-multilib \
  make \
  file \
  curl \
  ca-certificates \
  python2.7 \
  git \
  cmake \
  sudo \
  gdb \
  xz-utils


COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV RUST_CONFIGURE_ARGS --build=i686-unknown-linux-gnu --disable-optimize-tests
ENV SCRIPT python2.7 ../x.py test
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======

# FIXME(#59637) takes too long on CI right now
ENV NO_LLVM_ASSERTIONS=1 NO_DEBUG_ASSERTIONS=1
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
