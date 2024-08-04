# Rust Git Project

## Initial Description

This is a project that I have taken on to grow my knowledge of Software Development massively.

What makes this project impressive, in my opinion, is that I am programming in a new language (**Rust**) on a new IDE (**CLion** with Rust Plugin) with a new build system (**Cargo**), I am using new documentation software (**Markdown** on **Obsidian**), new project management software (**Jira**), and I am creating something that my current understand of is minimal (**Git**).

Through overcoming the challenges of all of these **new tools and technologies**, I aim to be an even more versatile programmer with a greater skill set.

## Architecture

So that I can run the project from anywhere, all data is stored locally. That being said, I have set up the layers such that the data storage layer is loosely coupled and easily replaceable with an API or some other data source.

The architecture that I have chosen has the following layers:

### Presentation
Terminal User Interface, communicating with Service and Infrastructure layer.

### Service
Conducts business logic, communicating with Data Access and Infrastructure layer.

### Data Access
Handles data retrieval and manipulation, communicating with Data Storage and Infrastructure layer.

### Data Storage
Reads and writes to and from files and directories, communicating only with Infrastructure layer.

### Infrastructure
Conducts small services that all layers may benefit from without an obvious layer to belong to.

These are all loosely coupled and **designed to be independently replaceable and scalable**, ensuring that changes in one layer do not adversely affect the others and allowing for easier maintenance and evolution of the system.