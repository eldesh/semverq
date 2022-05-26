FROM debian:bullseye

ARG IMAGE_VERSION

ARG DEBIAN_FRONTEND=noninteractive

# inherits version numbers to runtime
ENV IMAGE_VERSION=${IMAGE_VERSION}

COPY ./package.sh /package.sh

# OCI labels
LABEL org.opencontainers.image.authors="eldesh <nephits@gmail.com>"
LABEL org.opencontainers.image.source="https://github.com/eldesh/semverq/context/Dockerfile"
LABEL org.opencontainers.image.version="${IMAGE_VERSION}"
LABEL org.opencontainers.image.licenses="MIT License"
LABEL org.opencontainers.image.base.digest="sha256:62a747bf1719d2d37fff5670ed40de6900a95743172de1b4434cb019b56f30b4"
LABEL org.opencontainers.image.base.name="debian:bullseye"

RUN apt-get update \
 && apt-get install --no-install-recommends -y \
      sudo apt-utils git devscripts curl cdbs debhelper build-essential \
 ## avoid warning:
 ## > Use of uninitialized value $v in substitution (s///) at Dh_Lib.pm line 1729, <$fd> line 11.
 && sed -i -e '1729i\    return if not defined($v);' /usr/share/perl5/Debian/Debhelper/Dh_Lib.pm \
 && sudo apt-get autoremove \
 && sudo apt-get autoclean

ENV PATH=/root/.cargo/bin:${PATH}

CMD ["/package.sh"]

