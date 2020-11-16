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
- [Controlling Rover](#controlling-rover)
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

## Controlling Rover

When the programme starts you'll have to enter the upper-right coordinates of the rectangular world.
The maximum `x` or `y` value is 50.
So the biggest box you can build would be:

```
50 50
```

The next input is Rovers location and orientation.
This is inputed as the `x` and `y` position of the robot followed by the orientation.
The orientation is inputed as either `N`, `S`, `E` or `W`.
This would look as follows if `x` was 10, `y` 20 and Rover is facing West:

```
10 20 W
```

The next input will be the instructions for Rover's movement.
Their instruction is a string of the letters `L`, `R`, and `F` on one line.
So for example if you wanted Rover to move forward twice, then turn left, forward one more time and then turn right, it would look as follows:

```
FFLFR
```

After Rover's commands are processed it will output their final location.
They will also inform you if they are lost.
Rover will become lost once they leave the bounds of the grid.

A lost rover:

```
53
3 2 N
FRRFLLFFRRFLL
```

Will output:

```
3 3 N LOST
```

## Development

The simplest way to run the unit tests for this project is:

```
make test
```
