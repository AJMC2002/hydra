# HYDRA

An application for design and simulation of sprinkler systems built on Rust.

## About

The first part of the program allows the user to obtain fast calculations about the requirements of the system given the design parameters (risk level, equivalent length to account for pressure loss, etc.). It also provides the measurements of the sprinkler distribution in the system (i.e. separation between sprinklers and separation between branches).

Using a Finite Element approach to the incompressible Navier-Stokes equation, we can simulate the characteristics of water in the pipes given a design. The second part of the program uses this to show the possible results from the given design. The important measures are: fluid velocity and pressure inside pipes.

## Resources

- Rust Docs
- Slint Docs
- NFPA 13
- NFPA 20
