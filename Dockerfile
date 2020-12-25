FROM ubuntu:latest
MAINTAINER Luis Galup "legalup@yahoo.com"

RUN apt-get update && apt-get install -y software-properties-common python
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
	python3.4 \
	python3-pip

RUN apt-get install -y emacs

# Install OpenJDK-8
RUN apt-get update && \
    apt-get install -y openjdk-8-jre-headless && \
    apt-get clean;

# Install the plugin for javaws
# deb http://archive.ubuntu.com/ubuntu bionic universe
RUN apt-get update && \
    apt-get install icedtea-netx -y

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
RUN mkdir -p /home/galup/workspace
WORKDIR /home/galup/workspace

# lets do some git cloning, shall we
ARG GITPASSWD=ByteMe
RUN git clone https://legalup:"$GITPASSWD"@github.com/legalup/tc_greed.git


WORKDIR /home/galup
RUN wget https://github.com/shivawu/topcoder-greed/releases/download/2.0-RC/Greed-2.0-RC-7.1.0.jar


