#FROM rockylinux:latest
#FROM alpine:latest
FROM cirros:latest
MAINTAINER polarlabs

#RUN apk --update add ca-certificates
#RUN apk --update make
#RUN apk add gcompat

#RUN addgroup -S fleet && adduser -S fleet -G fleet

# RPM installation
#ADD https://github.com/osquery/osquery/releases/download/5.2.2/osquery-5.2.2-1.linux.x86_64.rpm /tmp/
#RUN rpm -i /tmp/osquery-5.2.2-1.linux.x86_64.rpm

# TAR.GZ installation
ADD https://github.com/osquery/osquery/releases/download/5.2.2/osquery-5.2.2_1.linux_x86_64.tar.gz /tmp/
RUN tar -xzf /tmp/osquery-5.2.2_1.linux_x86_64.tar.gz

#git clone --depth 1 --branch 5.2.2 https://github.com/osquery/osquery
#git clone --depth 1 --branch 1.1.0 https://github.com/osquery/osquery
#COPY ./build/binary-bundle/linux/fleet ./build/binary-bundle/linux/fleetctl /usr/bin/

#USER fleet
CMD /bin/ash
