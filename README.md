# Mars Rover
This is a set of implementations of the Mars Rover programming kata.
---

## Table of Contents

- [Getting Started](#getting-started)
    - [Quick start](#quick-start)
- [Controlling Rover](#controlling-rover)
- [Development](#development)

## Getting Started

First clone this repo and cd into the directory:

```
git clone https://github.com/carlosroman/mars-rover.git
cd mars-rover
```

### Quick start

For the quickest start you just need:

* Make
* Docker

With those two items you can then just run the following to start the application.
To quick start to the golang version run:

```
make quick-start/go
```

You should see some out put on the console and once everything stops you should be ready to start providing input.

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
cd <language>
make test
```
