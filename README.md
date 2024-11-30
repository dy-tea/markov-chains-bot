# Markov Chains Discord Bot
This is a Discord Bot that generates text using Markov chains. Models can be generated using plain text files and queried by their tokens. See [here](https://github.com/krypt0nn/markov-chains) for the implementation of the markov chains generator.

## Supported commands
- `/sysinfo`
- `/query`
- `/params`
    - `set`
    - `reset`
    - `show`
- `/model`
    - `list`
    - `info`
    - `load`
    - `build`
    - `fromscratch`

## Usage
Use `/query <QUERY>` to generate text with the currently loaded model.

Use `/model list` to list the available models.

Use `/model load <MODEL_NAME>` to load a different model.

Use `/model fromscratch <URL>` to load a text file from that url and create a model from it.
