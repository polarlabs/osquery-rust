FROM rockylinux:latest
MAINTAINER polarlabs

LABEL description="Testing osquery image build"

ENV USER=osquery \
    UID=1001 \
    GROUP=osquery \
    GID=1001

ENV RELEASE=5.2.2
ENV BUILD=$RELEASE-1
ENV UPSTREAM_PKG=osquery-$BUILD.linux.x86_64.rpm
ENV INSTALL_CMD="rpm --install /tmp/$UPSTREAM_PKG"

ADD https://github.com/osquery/osquery/releases/download/$RELEASE/$UPSTREAM_PKG /tmp/

RUN groupadd --gid $GID $GROUP && \
    useradd --uid $UID --gid $GROUP $USER && \
    $INSTALL_CMD && \
    rm --force /tmp/$UPSTREAM_PKG

USER $USER
CMD ["/bin/bash"]
