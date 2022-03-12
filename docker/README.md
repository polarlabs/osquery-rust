Because the osquery project does not regularly update its docker images.

We want to please osquery extension developers, wether they use osquery-rust bindings or other bindings.

If you are missing something in this image, please tell us.

# Why we build

To ensure quality of osquery-rust we have to test it together with osquery. To facilitate testing across various 
versions of osquery and different platforms, docker container come into play.

Developers of osquery extensions, whether they use osquery-rust or not have similar requirements and benefit from 
osquery docker images provided by polarlabs.

# How we build

https://api.github.com/repos/osquery/osquery/releases/latest

# How we publish

docker push polarlabs/osquery:5.2.2-rockylinux-latest
