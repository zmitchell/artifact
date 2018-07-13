# REQ-learn

Welcome to the artifact tutorial! This file is written just like artifact
markdown files are. Artifact files can be written in a range of formats, the
currently supported ones being markdown, toml and yaml.

Artifacts can be a requirement (REQ), design-specification (SPC)
or test (TST)

Artifacts are defined by specifying their name like so:

```
# REQ-NAME
<regular markdown section here>
```

This particular artifact is a requirement, therefore it begins with
"REQ". After REQ there is a "-" and then the name of the requirement.

Unlike many requirements tracking tools, artifact encourages the use
of human-readable names. We will talk more about this in this tutorial.

# REQ-markdown
partof: REQ-learn
###

Artifact files like this one are written in a slightly extended markdown format
you can read more about it here: http://commonmark.org/help/tutorial/

The "extended" part is that artifact treats the following syntax as special:
```
# ART-name
<optional yaml section here>
###
<regular markdown section here>
```

Where `ART` is one of `REQ`, `SPC`, `TST` and `<optional yaml here>` is a few
items like `partof` and `done` fields. We will get to those later.


# SPC-learn
partof: REQ-markdown
###

Anything starting with SPC is a design specification. Comparing them:

Requirements (REQ) should be used for
- detailing what you want your application to do
- what the architecture of your applicaiton should be


Specifications (SPC) should be used for
- how you intend to write your application (lower level details).

There are also tests (TST) which we will learn about later.

# SPC-partof
partof: REQ-learn
###

Artifact uses the names of artifacts to automatically link them and track
progress. This makes it easy for the user to intuitively link together
requirements with their specification and reduces boilerplate.

[[SPC-learn]] is automatically a "partof" [[REQ-learn]] because the names after
the type are the same ("-learn")

So far we have:
```
REQ-learn <------ SPC-learn
    ^-- REQ-toml <---/
    |
    \---REQ-partof <-- SPC-partof
```

# SPC-example
only used as an example for [[TST-partof]]


# TST-partof
partof:
- SPC-learn
- SPC-example
###

First of all, this is how you specify a TST.

Also, notice the `partof` field is a list to specify more than one artifact.

# SPC-valid

There are only a few rules for defining artifacts:
 - case is ignored for all names
 - names cannot overlap, even in different files
 - all names must start with either REQ, SPC or TST
 - artifact names must follow [[SPC-links]] below.

# SPC-links
partof: SPC-partof
###

Here is a helpful graph of valid relations:
```
  REQ <-- SPC <-- TST
```

Therefore:
- A REQ can be partof a REQ only
- A SPC an be partof a REQ or SPC
- A TST can be partof a REQ, SPC or TST

# SPC-tst
TST is used to document test design and is the only way that an artifact can be
considered "tested" (besides the `done` field).

Artifact makes it easy to track the "progress" of your application because `art
ls` (and the web-ui) gives you easy to easy to read completion and tested
percentages for all your artifacts based on which ones are implemented in
source code (more on that later).
