# Osquery as a container

This part of the project provides a set of osquery images via a public registry. For the time being, 
the project supports [Docker Hub](https://hub.docker.com/u/polarlabs).

# Reasoning for providing images with Osquery preinstalled

You might wonder why such an image might be of interest. In fact, deploying Osquery within a container 
doesn't seem to make much sense. However, from a developer's perspective, being able to test `osquery-rust` 
with its interface to Osquery, is a prerequisite to ensure quality of `osquery-rust`. Containers covering 
different versions of Osquery as well as various platforms are the way to go.

Also, developers looking for implementing their own extension will benefit from [polarlabs/osquery at Docker Hub](https://hub.docker.com/r/polarlabs/osquery).

# Reasoning behind building and publishing custom images

Because the Osquery project does not regularly update its Docker images, we strive to fill the gap. We still 
have to automate the process as far as possible.

Extension developers, whether they use `osquery-rust` or not have similar requirements and benefit from 
the Docker images provided.

If you are missing something in this image, please tell us.

# How we build

todo: this has to be automated with GitHub actions.

docker build --tag polarlabs/osquery:5.2.2-rockylinux-latest .

docker build --tag polarlabs/osquery:5.2.2-debian-stable-slim .

https://api.github.com/repos/osquery/osquery/releases/latest

# How we publish

docker push polarlabs/osquery:5.2.2-rockylinux-latest

# How we use

    $ pwd
    $HOME/Documents/DevProjects/osquery-rust
    $ docker build -f ./docker/Dockerfile .
    Sending build context to Docker daemon  375.1MB
    Step 1/3 : FROM polarlabs/osquery:5.2.2-debian-stable-slim
    ---> 0899d549e8be
    Step 2/3 : COPY target/debug/table-proc-meminfo /tmp/
    ---> Using cache
    ---> 8c0934457502
    Step 3/3 : CMD ["osqueryi", "--extension", "/tmp/table-proc-meminfo"]
    ---> Using cache
    ---> c8a42c12bc74
    Successfully built c8a42c12bc74
    $

# Automated build

1. First challenge: track Osquery repo regarding
   - commits on master
   - new releases

## To get commits

Fork manually once

Repeatedly:

1. Determine last commits before sync

https://docs.github.com/en/rest/commits/commits?apiVersion=2022-11-28#list-commits

2. Sync fork with Upstream via GitHub API in GitHub Actions:

https://dev.to/n3wt0n/3-ways-to-sync-a-forked-repository-on-github-automatically-cfd

3. Determine last commits after sync?

before != after

=> Build with rev#

How to build?

Lets focus on Linux X86 first.



## To get releases

Use API.

All releases:

    curl -s https://api.github.com/repos/osquery/osquery/releases

Latest release:

    curl -s https://api.github.com/repos/osquery/osquery/releases/latest


https://stackoverflow.com/questions/58465057/trigger-a-github-action-when-another-repository-creates-a-new-release

https://stackoverflow.com/questions/65619464/trigger-a-github-action-on-a-foreign-repo-update-push

What else do we need?

1. A list of supported Linux distributions we build Osquery for.
2. A list of maintained version of supported Linux distributions.
3. A template for Dockerfile, i.e. using ARGS to control the build process.
4. Use metadata in Dockerfile.

# Links

[Building Rust binaries in CI that work with older GLIBC](https://kobzol.github.io/rust/ci/2021/05/07/building-rust-binaries-in-ci-that-work-with-older-glibc.html)
