# Telemetra

Telemetra is an extensible and lightweight ecosystem designed to
collect, analyze, and report all kinds of data from any number of
sources.

The Telemetra ecosystem is composed of:

- **Telemetra Actor**: A single actor that provides a set of services
  generating data streams (e.g., logging, tracing, or information
  services).  Each Telemetra Actor advertises its available services
  to the Telemetra Leader.  Multiple actors can run on the same
  machine (node), and each actor may provide multiple services.

- **Telemetra SDK**: A software development kit used to write
  Telemetra Actors and instrument applications with Telemetra
  capabilities.

- **Telemetra Leader**: Manages the lifecycle of services and collects
  data from the Actors.  For each service, the Leader provides:
  - **Reporter**: Generates reports in various formats (PDF, HTML,
  plaintext, CSV, etc.).
  - **Notifier**: Sends event messages to other processes (typically a
  client), for example when a new actor registers/unregisters or when
  new data is received.
  
- **Telemetra Client**: A secure client for managing the Telemetra
  Leader. It includes:
  - **Telemetra CLI**: Command-line interface.
  - **Telemetra Web**: Graphical web interface.
