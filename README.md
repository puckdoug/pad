# pad (rpad, lpad)

Command-line utility to pad text or numbers easily. This is installed as 3
separate commands: `pad`, `lpad`, and `rpad`. It reads a list of words from
stdin and pads them with the padding specified (default 0) on left or right.

With no arguments, it will pad to the width of the longest word. However, this
requires that the entire input file be read into memory so it can be processed.
To handle files more efficiently, if width is specified the input will be
processed line-by-line, allowing for arbitrarily large files to be processed.

## Usage

```shell

pad [ --lpad <count> <character> ] [ --rpad <count> <character> ] < <file>
pad [ --lpad <count> <character> ] [ --rpad <count> <character> ] <word> [ <word> ... ]
lpad [ <count> <character> ] [ --rpad <count> <character> ] < <file>
lpad [ <count> <character> ] [ --rpad <count> <character> ] <word> [ <word> ... ]
rpad [ <count> <character> ] [ --lpad <count> <character> ] < <file>
rpad [ <count> <character> ] [ --lpad <count> <character> ] <word> [ word ...]

```
