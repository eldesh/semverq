% semverq(1)
% eldesh <nephits@gmail.com>
% May 2022


# NAME

semverq - a cli utility for accessing semver structures.


# SYNOPSYS

**semverq** [**-i** *input*] [**-j**]\
**semverq** [**-i** *input*] [**-m** *match*]\
**semverq** [**-i** *input*] [**-q** *query*]


# DESCRIPTION

**semverq** is a cli utility for:

- Validating semver format
- Accessing semver structures
- Convert semver to json


# OPTIONS
**-i &lt;input&gt;**
:  version string

**-j**
:  Convert the input string to a json

**-m &lt;match&gt;**
:  Checking for a match

**-q &lt;query&gt;**
:  Expand accessors

**-V**, **--version**
:  Print version information

**-h**, **--help**
:  Print help information


# EXIT STATUS
**0**
:   Success

**150**
:   Invalid semver

**151**
:   Invalid version requirement

**152**
:   Not matching to the version requirement


# EXAMPLES

## Validation

For `-i` option, semverq validates the input string as a semver.

```
$ semverq -i '1.2.3-beta+dev-armhf'
(nothing is output)
$ echo $?
0
```

```
$ semverq -i '1.2.3-beta+dev+armhf'
invalid format as semver: 1.2.3-beta+dev+armhf
$ echo $?
150
```

If the `-i` option is not specified, semverq reads string from standard input.

```
$ semverq <<<'1.2.3-input+stdin'
$ echo $?
0
```

## Convert to json

For `-j` option, semverq converts the input to json.

```
$ semverq -j -i '1.2.3-beta+36a1d2f'
{
  "major": 1,
  "minor": 2,
  "patch": 3,
  "pre-release": "beta",
  "build": "36a1d2f"
}
```

If `pre-release` or `build` is not exists, the values of their fields will be `null`.

```
$ semverq -j -i '1.2.3'
{
  "major": 1,
  "minor": 2,
  "patch": 3,
  "pre-release": null,
  "build": null
}
```

## Accessing to parts

For `-q` option, access semver parts.

Get only major version:

```
$ semverq -q '.major' -i '1.2.3-beta+36a1d2f'
1
```

Build a json object with `pre-release` and `build`:

```
$ semverq -q '{ "pre-release": ".pre-release", "build": ".build" }' -i '1.2.3-beta+36a1d2f'
{ "pre-release": "beta", "build": "36a1d2f" }
```

### Accessor

Supported accessors are below:

**.major**
:  Major version.

**.minor**
:  Minor version.

**.patch**
:  Patch version.

**.pre-release** or **.pre**
:  pre-release version.

**.build**
:  Build metadata.

**.version-core**
:  Shorthand for *.major..minor..patch*.


## Checking for a match

For `-m` option, semverq checking for a match to given requirement.
The requirement is constructed from comma separated version constraint.
This matching syntax and rules are same as in [Rust's package system](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html).


Matching case:

```
$ semverq -m '>1.2.1, <=2.0.0' -i '1.2.3'
$ echo $?
0
```

Not matching case:

```
$ semverq -m '^1.2.1, 1.3.*' -i '1.2.3-beta+dev-armhf'
Version (1.2.3-beta+dev-armhf) does not match that requirement (^1.2.1, 1.3.*).
$ echo $?
152
```

Invalid requirement:

```
$ semverq -m '>1.2.1, <=2.0.0.' -i '1.2.3'
invalid format as version requirement: >1.2.1, <=2.0.0.
$ echo $?
151
```

# LICENSE

Copyright (c) eldesh <nephits@gmail.com>


Released under the MIT license


https://opensource.org/licenses/mit-license.php

