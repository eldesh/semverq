#!/bin/bash
set -eux

ftparchive() {
  (
    cd $1
    apt-ftparchive sources  . | gzip -c9 > Sources.gz
    apt-ftparchive packages . | gzip -c9 > Packages.gz
    apt-ftparchive contents . | gzip -c9 > Contents-$(dpkg --print-architecture).gz
    apt-ftparchive release  . | gzip -c9 > Release.gz
  )
}

build_src_pkg() {
  mkdir -p $1
  cp -R /semverq $1/semverq-${DEB_VERSION}
  (
    cd $1/semverq-${DEB_VERSION} && debuild -uc -us
  )
  rm -rf $1/semverq-${DEB_VERSION}
}

deploy_pkg() {
  rm -rf $2
  mv $1 $2
  chown -R 1000:1000 $2
}

build_src_pkg /work
ftparchive /work
deploy_pkg /work /semverq/package

