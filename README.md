# One of many possible agent based infectious disease models

This is just a dumb test of how to do an agent based infectious disease model

## Description

Agents can be one of three statuses

  * Susceptible
  * Infected
  * Recovered

At each step an agent who is

  * Infected -> Recover at a given probability, or remain infected
  * Susceptible -> Become infected at the infection probability

The infection probability is calculated as

```
(1 - (1 - p)^n) * t
```
Where `p` is the proportion of agents infected at the current step, 
`n` is a given number of interactions, and `t` the given transmission probability


## Disclaimer

I don't know what I'm doing :)
