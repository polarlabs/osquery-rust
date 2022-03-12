FROM rockylinux:latest
MAINTAINER polarlabs

LABEL description "Testing osquery image build"

ENV USER=osquery \
    UID=1001 \
    GROUP=osquery \
    GID=1001

ADD https://github.com/osquery/osquery/releases/download/5.2.2/osquery-5.2.2-1.linux.x86_64.rpm /tmp/

RUN groupadd --gid $GID $GROUP && \
    useradd --uid $UID --gid $GROUP $USER && \
    rpm --install /tmp/osquery-5.2.2-1.linux.x86_64.rpm && \
    rm --force /tmp/osquery-5.2.2-1.linux.x86_64.rpm

USER $USER
CMD ["/bin/bash"]
