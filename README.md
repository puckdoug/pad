# pad (rpad, lpad)

Command-line utility to pad text or numbers easily. This is installed as 3
separate commands: `pad`, `lpad`, and `rpad`. It reads a list of words from
stdin and pads them with the padding specified (default 0) on left or right.

With no arguments, it will pad to the width of the longest word. However, this
requires that the entire input file be read into memory so it can be processed.
To handle files more efficiently, if width is specified the input will be
processed line-by-line, allowing for arbitrarily large files to be processed (not yet implemented).

## Usage

The help output on the command line:

```shell
Usage: pad|lpad|rpad [ <width> [ <pad-str> ] ] [options] [tokens] [ < input ]
Options:
  -l, --left  [ <width> [ <pad-str> ] ]  Pad the left side
  -r, --right [ <width> [ <pad-str> ] ]  Pad the right side
  -h, --help                             Display this help message
```

Expanded usage:

```shell
pad [ --left <count> <character> ] [ --right <count> <character> ] < <file>
pad [ --left <count> <character> ] [ --right <count> <character> ] <word> [ <word> ... ]

lpad [ <count> <character> ] [ --right <count> <character> ] < <file>
lpad [ <count> <character> ] [ --right <count> <character> ] <word> [ <word> ... ]

rpad [ <count> <character> ] [ --left <count> <character> ] < <file>
rpad [ <count> <character> ] [ --left <count> <character> ] <word> [ word ...]
```

## Examples

Pad a list of words with zeros from the left to the width of the widest word (default case):

```shell
$ lpad < words.txt
$ lpad 0 0 < words.txt
$ pad --left 0 0 < words.txt
$ pad --left < words.txt
$ pad < words.txt
$ lpad these are a few words to pad
$ pad these are a few words to pad
```

Note that in most cases pad and lpad are functiaonlly equivalent. However, when specifying a width and token explicitly, pad will not assume that the first two characters are arguments, while lpad will do so. This leads to differences, as below:

```shell
$ pad 10 x these are a few words to pad
00010
0000x
these
00are
0000a
00few
words
000to
00pad

$ lpad 10 x these are a few words to pad
xxxxxthese
xxxxxxxare
xxxxxxxxxa
xxxxxxxfew
xxxxxwords
xxxxxxxxto
xxxxxxxpad

# and for completeness
$ rpad 10 x these are a few words to pad
thesexxxxx
arexxxxxxx
axxxxxxxxx
fewxxxxxxx
wordsxxxxx
toxxxxxxxx
padxxxxxxx
```

Multibyte characters are supported, but font widths mean that this doesn't appear correct visually, at least on my screen. There may be a fixed width unicode font where this outputs strictly in columns, though I expect the latin characters would seem quite spread in that case.

At the moment, multiple characters is not. Or more precisely, it does something, but it's probably not what you want. Fixing this is planned.

```shell
$ lpad 10 ウ these are a few words to pad
ウウウウウthese
ウウウウウウウare
ウウウウウウウウウa
ウウウウウウウfew
ウウウウウwords
ウウウウウウウウto
ウウウウウウウpad

$ lpad 10 abc these are a few words to pad
abcabcabcabcabcthese
abcabcabcabcabcabcabcare
abcabcabcabcabcabcabcabcabca
abcabcabcabcabcabcabcfew
abcabcabcabcabcwords
abcabcabcabcabcabcabcabcto
abcabcabcabcabcabcabcpad
```
