# Dining Philosophers Problem - Rust
Five philosophers dine together at the same table. Each philosopher has their own place at the table. There is a fork between each plate. The dish served is a kind of spaghetti which has to be eaten with two forks. Each philosopher can only alternately think and eat. Moreover, a philosopher can only eat their spaghetti when they have both a left and right fork. Thus two forks will only be available when their two nearest neighbors are thinking, not eating. After an individual philosopher finishes eating, they will put down both forks.

<hr>

## Example STDOUT

![](stdout.png)

<hr>

## Sequence Diagram

```mermaid
sequenceDiagram
    %% Define custom styles for philosophers and forks
    participant Pythagoras as "Pythagoras 🍽️":::philosopher
    participant Plato as "Plato 🍽️":::philosopher
    participant Hypatia as "Hypatia 🍽️":::philosopher
    participant Socrates as "Socrates 🍽️":::philosopher
    participant Aristotle as "Aristotle 🍽️":::philosopher
    participant Fork0 as "Fork 0":::fork
    participant Fork1 as "Fork 1":::fork
    participant Fork2 as "Fork 2":::fork
    participant Fork3 as "Fork 3":::fork
    participant Fork4 as "Fork 4":::fork

    %% Philosophers pick up forks (indicating left and right fork)
    Pythagoras ->> Fork0: Pick up Left Fork
    Pythagoras ->> Fork4: Pick up Right Fork
    Plato ->> Fork1: Pick up Left Fork
    Plato ->> Fork0: Pick up Right Fork
    Hypatia ->> Fork2: Pick up Left Fork
    Hypatia ->> Fork1: Pick up Right Fork
    Socrates ->> Fork3: Pick up Left Fork
    Socrates ->> Fork2: Pick up Right Fork
    Aristotle ->> Fork4: Pick up Left Fork
    Aristotle ->> Fork3: Pick up Right Fork

    %% Philosophers wait if a fork is unavailable
    Plato ->> Fork0: Wait for Right Fork (Blocked)
    Hypatia ->> Fork1: Wait for Right Fork (Blocked)
    Socrates ->> Fork2: Wait for Right Fork (Blocked)
    Aristotle ->> Fork3: Wait for Right Fork (Blocked)

    %% Philosophers can eat after both forks are picked up
    Pythagoras ->> Pythagoras: Eating 🍝
    Plato ->> Plato: Eating 🍝
    Hypatia ->> Hypatia: Eating 🍝
    Socrates ->> Socrates: Eating 🍝
    Aristotle ->> Aristotle: Eating 🍝

    %% Philosophers release forks
    Pythagoras ->> Fork0: Release Left Fork
    Pythagoras ->> Fork4: Release Right Fork
    Plato ->> Fork1: Release Left Fork
    Plato ->> Fork0: Release Right Fork
    Hypatia ->> Fork2: Release Left Fork
    Hypatia ->> Fork1: Release Right Fork
    Socrates ->> Fork3: Release Left Fork
    Socrates ->> Fork2: Release Right Fork
    Aristotle ->> Fork4: Release Left Fork
    Aristotle ->> Fork3: Release Right Fork

    %% Repeat cycle of picking up forks, eating, and releasing forks
    Pythagoras ->> Fork0: Pick up Left Fork
    Pythagoras ->> Fork4: Pick up Right Fork
    Plato ->> Fork1: Pick up Left Fork
    Plato ->> Fork0: Pick up Right Fork
    Hypatia ->> Fork2: Pick up Left Fork
    Hypatia ->> Fork1: Pick up Right Fork
    Socrates ->> Fork3: Pick up Left Fork
    Socrates ->> Fork2: Pick up Right Fork
    Aristotle ->> Fork4: Pick up Left Fork
    Aristotle ->> Fork3: Pick up Right Fork
    Note over All: Repeat cycle with no delays

```
<hr>

## Build
```
cargo build
```

## Run
```
./target/debug/dining-philosophers
```