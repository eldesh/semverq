# SemVerq

Semverq is a cli utility for:

- Validating semver format
- Accessing semver structures
- Convert semver to json

# Examples

## Validation

semverq always validates the input string as a semver.

```
$ semverq -i '1.2.3-beta+dev-armhf'
(nothing is output)
$ echo $?
0
```

```
$ semverq -i '1.2.3-beta+dev+armhf'
Error: unexpected character '+' after build metadata
$ echo $?
1
```

## Convert to a json

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

```
$ semverq -q '.major' -i '1.2.3-beta+36a1d2f'
1
```

```
$ semverq -q '{ "pre-release": .pre-release, "build": .build }' -i '1.2.3-beta+36a1d2f'
{ "pre-release": "beta", "build": "36a1d2f" }
```


