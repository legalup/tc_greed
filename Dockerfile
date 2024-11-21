FROM ubuntu:latest
MAINTAINER Luis Galup "legalup@yahoo.com"

RUN apt-get update && apt-get install -y software-properties-common
RUN add-apt-repository universe
RUN add-apt-repository restricted
RUN add-apt-repository multiverse

# Install basic python and catkin tools and create a workspace.
RUN apt-get update && apt-get install -qq --no-install-recommends \
    	wget \
	curl \
        apt-transport-https \
        apt-utils \
	git \
	less \
	findutils \
	openssh-client \
	locate \
	cmake \
	python3.4 \
	python3-pip

#installs g++ 
RUN apt-get update && apt-get install -y build-essential

#installs necessary for FlapHero
RUN apt-get update && apt-get install -y libassimp-dev libglfw3-dev

RUN apt-get install -y emacs


#RUN apt-get update && \
#    apt-get install -y openjdk-8-jre-headless && \
#    apt-get clean;

# Install the plugin for javaws
# deb http://archive.ubuntu.com/ubuntu bionic universe
#RUN apt-get update && \
#    apt-get install icedtea-netx -y

#install firefox
RUN apt-get update && apt-get install -y firefox

# making my favorite user
RUN useradd -ms /bin/bash galup
RUN apt-get update && apt-get install -y sudo  && adduser galup sudo

# Replace 1000 with your user / group id
RUN export uid=1000 gid=1000
RUN mkdir -p /home/developer
RUN echo "developer:x:${uid}:${gid}:Developer,,,:/home/developer:/bin/bash" >> /etc/passwd
RUN echo "developer:x:${uid}:" >> /etc/group
RUN echo "developer ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
RUN chmod 0440 /etc/sudoers
RUN chown ${uid}:${gid} -R /home/developer

USER galup
ENV HOME /home/galup
RUN mkdir -p /home/galup/workspace/tc_greed
WORKDIR /home/galup/workspace

# Install OpenJDK-8
#RUN wget https://javadl.oracle.com/webapps/download/AutoDL?BundleId=244575_d7fc238d0cbf4b0dac67be84580cfb4b


# lets do some git cloning, shall we
#ARG GITPASSWD=ByteMe
#RUN git clone https://legalup:"$GITPASSWD"@github.com/legalup/tc_greed.git

# the following get plywood setup
#RUN git clone https://github.com/arc80/plywood
#WORKDIR /home/galup/workspace/plywood
#RUN cmake -DPRESET=make -P Setup.cmake
#WORKDIR /home/galup/workspace/plywood/repos
#RUN git clone https://github.com/arc80/FlapHero
#WORKDIR /home/galup/workspace/plywood
#RUN ./plytool codegen ; ./plytool build --auto glfwFlap ; ./plytool extern select --install assimp.apt ; \
#./plytool extern select --install soloud.source ; ./plytool extern select --install glfw.apt
#RUN ./plytool build --auto glfwFlap ; ./plytool build


#WORKDIR /home/galup
#RUN wget https://github.com/shivawu/topcoder-greed/releases/download/2.0-RC/Greed-2.0-RC-7.1.0.jar

RUN git config --global user.email "legalup@pm.me" && git config --global user.name "luis galup"

WORKDIR /home/galup/workspace/tc_greed
USER root
#RUN dpkg -i OpenWebStart_linux_1_4_0.deb

LABEL org.opencontainers.image.source=https://github.com/rust-lang/docker-rust


ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.82.0

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='6aeece6993e902708983b209d04c0d1dbb14ebb405ddb87def578d41f920f56d' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='3c4114923305f1cd3b96ce3454e9e549ad4aa7c07c03aec73d1a785e98388bed' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='1cffbf51e63e634c746f741de50649bbbcbd9dbe1de363c9ecef64e278dba2b2' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='0a6bed6e9f21192a51f83977716466895706059afb880500ff1d0e751ada5237' ;; \
        ppc64el) rustArch='powerpc64le-unknown-linux-gnu'; rustupSha256='079430f58ad4da1d1f4f5f2f0bd321422373213246a93b3ddb53dad627f5aa38' ;; \
        s390x) rustArch='s390x-unknown-linux-gnu'; rustupSha256='e7f89da453c8ce5771c28279d1a01d5e83541d420695c74ec81a7ec5d287c51c' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.27.1/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

USER galup
