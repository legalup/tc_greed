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



