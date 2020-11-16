# go-rover
A simple implementation of the martian robots kata.
![Build](https://github.com/carlosroman/go-rover/workflows/Run%20tests/badge.svg)
[![Coverage Status](https://coveralls.io/repos/github/carlosroman/go-rover/badge.svg?branch=master)](https://coveralls.io/github/carlosroman/go-rover?branch=master)
---

## Table of Contents

- [Getting Started](#getting-started)
    - [Quick start](#quick-start)
    - [Running the application without Docker](#running-the-application-without-docker)
    - [Building standalone binary](#building-standalone-binary)
- [Development](#development)

## Getting Started

First clone this repo and cd into the directory:

```
git clone https://github.com/carlosroman/go-rover.git
cd go-rover
```

### Quick start

For the quickest start you just need:

* Make
* Docker

With those two items you can then just run the following to start the application:

```
make quick-start
```

You should see some out put on the console and once everything stops you should be ready to start providing input.

### Running the application without Docker

If you want to run the application without Docker, then you need the following:

* Make
* Golang 1.4+

The application can then be run by using the following command

```
make start
```

### Building standalone binary

If you want to run the application without Docker, then you need the following:

* Make
* Golang 1.4+

```
make build
```

After the command has finished you'll have a binary called `rover` in directory called `bin`.
By default, this binary will be built to run on the local OS and architecture.

## Development

The simplest way to run the unit tests for this project is:

```
make test
```
