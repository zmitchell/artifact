# Artifact Intro

This is an introduction to how we start integrating the Artifact tool into our
design and development workflow. It is intended to be interactive, so please
follow along with everything installed!

> **Exercise 1: ensuring your environment works:**
>
> You should have at least done the [Starting Project](./StartingProject.html)
> chapter before attempting this one.
>
> Run `art ls`, you should see something like:
>
> ```
> spc% tst%  | name         | parts
> 0.0  0.0   | REQ-purpose  |
> ```

## Converting our README.md into an artifact.

The first thing we want to do is use the `README.md` file we have already been writing
as our artifact file.

To do this, let's make a couple of changes:
- Move our `README.md` into `design/purpose.md`
- Clean up the headers so they are artifacts.

To move your README.md, simply type:
```
mv README.md purpose.md
```

> **Check In:** run `art ls`. It shows nothing because we have not specified
> any artifacts.

We now need to convert our headers into artifacts. Let's start with our purpose.
Change the `# Purpose` line to `# REQ-purpose`. Your file should now look something like:

```
# REQ-purpose
Write a flash card quizzer ...

```

Do the same thing to your specifications:
- `# Execution Method` -> `# SPC-cli`
- `# Final Results` -> `# SPC-report`
- `# Question File Format` -> `# SPC-format`

Now `art ls` should show:
```bash
$ art ls
spc% tst%  | name         | parts
0.0  0.0   | REQ-purpose  |
0.0  0.0   | SPC-cli      |
0.0  0.0   | SPC-format   |
0.0  0.0   | SPC-report   |
```

This is closer, but notice that none of them are linked. Let's fix that.

For `SPC-cli` make it look like this:

```
# SPC-cli
partof:
- REQ-purpose
###
The minimum viable product ...
```

Do the same for `SPC-format` and `SPC-report`, also making them partof
`REQ-purpose`. You should now have:
```
$ art ls
spc% tst%  | name         | parts
0.0  0.0   | REQ-purpose  | SPC-cli, SPC-format, SPC-report
0.0  0.0   | SPC-cli      |
0.0  0.0   | SPC-format   |
0.0  0.0   | SPC-report   |

```

--------------------------------------------------
## Tutorial Stage 3: detailed design and test design of the loading function
> **Run `art tutorial 3` to reset the local directory to this stage**

A few changes have been made to your local directory:
 - [`design/load.toml`](load-1.toml) has been created

> ### Exercise 1:
> Read through [`design/load.toml`](load-1.toml) and see if the general plan
> makes sense to you. What would you change? Feel free to make any edits you
> think should be made. You can always return it to it's original state with
> `art tutorial 3`

The first task we are going to address is how we load the questions into
the program. This is all defined under SPC-load. Run:
```
    art ls SPC-cmd -l
```

From there you can see the parts that have to be implemented for SPC-load
to be considered done. Note that SPC-LOAD was auto-created because it is a
parent of other artifacts.

> ### Exercise 2:
> Explore each part of SPC-load using the `art ls` and/or `art serve` cmd

`load.toml` details quite a bit of the design specifications, risks and tests
in order to implement this project. Let's actually get to work and start
coding.


--------------------------------------------------
## Tutorial Stage 4: writing and linking code
> **Run `art tutorial 4` to start this stage of the tutorial**

A few changes have been made to your local directory:
 - The `flash/` directory has been created with:
    - two files, `__init__.py` and [`load.py`](load-1.py)
    - The `tests/` directory, containing `__init__.py`,
      [`test_load.py`](test_load.py) and [`example.csv`](test_data.csv)
 - [`.art/settings.toml`](settings-2.toml) was updated to include the
   `code_paths` variable

> Note: for python, a directory with an `__init__.py` file is called a "module"
> and is python's packaging mechanism.

Take a look at [`flash/load.py`](load-1.py), which contains the machinery for
loading the flash-cards file. Notice the various `#SPC-...` tags located in the
documentation strings. These tags are how artifact knows which artifacts are
implemented and where and can mark implemented artifacts as done.

Additionally, an artifact is only considered "tested" when it's TST parts are
considered done.

Run the command

    art ls SPC-load -l

Notice that it is now "defined-at" [`flash/load.py`](load-1.py). Go to
where it says it is implemented and confirm that the information is correct.

Head to [`flash/tests/test_load.py`](test_load.py) and notice that similar tags
can be found there for TST artifacts.

### Exercises
1. run `art ls ARTIFACT` on an artifact that is tagged in source. Now
   change the tag (i.e. `SPC-load` -> `SPC-format`) and run it again. Did the
   completeness change?
2. Do the same thing for an arifact in the `partof` field for a file in
  `design/`. Notice that invalid names blink red on your terminal and you get
   WARN messages. You can use this feature to help you ensure your artifact
   links are correct.
3. We will be learning about `art check` in the next step. Try it now with
   the changes you've made

--------------------------------------------------
## Tutorial Stage 5: handling errors
> **Run `art tutorial 5` to start this stage of the tutorial**

A few changes have been made to your local directory:
 - [`design/load.toml`](load-2.toml) has been changed to have a bunch of errors
 - [`src/load.py`](load-2.py) has been changed to include a few errors as well.

So far in the tutorial things have been done correctly -- but what if you
are new, or what if you have to refactor?

Here we are in the middle of refactoring our code and requirements a bit... but
we've messed some things up. It's your job to fix them. How to begin?

First of all, we can use what we already know. `art ls` can help a lot for
refactors. It can answer the question "why is that SPC at 0%? It is implemented
somewhere!"

Well, let's try it for this project:

```
    # note: -OD displayes "partof | defined-at" instead of "parts | defined-at"
    art ls -OD
```

Things don't look quite as done as they used to. In particular notice:
- `SPC-validate` is 100% tested but 0% done (that's not right!)
- `REQ-learning` is also 100% tested and 0% done
- `REQ-purpose` has droped from 75% done to only 25% done

`art ls` can help you do this kind of investigation, but if you are refacting
then tracing errors this way is tedious. Those artifacts used to be
implemented... isn't there some way to find where they used to be tagged?

There is, run `art check` is the commnad you want. It analyzes your project for
errors and displays them in a way that makes them easier to fix. Some of the
errors it finds are:
 - invalid `partof` fields: if you've renamed (or misspelled) an artifact but
    forgot to update artifacts that were parts of it, this will help you.
 - dangling locations in code: you might THINK writing `#SPC-awesome-func`
    in your code links to something, but unless that spec actually exists
    it isn't doing anything. `art check` has your back.
 - recursive links: artifact's completeness algorithm doesn't work if there are
    recursive partof links (i.e. A is partof B which is partof A)
    `art check` will help you narrow down where these are comming from.
 - hanging artifacts: if you've written a SPC but haven't linked
    it to a REQ, then you probably want to (otherwise what exactly are you
    specifying?). The same goes for tests that are not testing any specs or
    risks.

> ### Exercise:
> use `art check` to find errors and fix them. Keep running
> `art check` and fixing errors until there are no errors, then run
> `art ls` to see if the current status makes sense.

--------------------------------------------------
## Documenting and Hosting your own project
To start documenting your own project, run `art init` in your project and
edit `.art/settings.toml` with the paths on where to find your
design docs and code.

Have your build system export your design documents as html for easy viewing.
See: https://github.com/vitiral/artifact/blob/master/docs/ExportingHtml.md

--------------------------------------------------
## Additional Resources

This tutorial gave you a good feature overview of artifact but you are probably
hungry to know quality best practices (you are, aren't you?). No worries!
The author of this tool has written an EXTREMELY SHORT ebook for just that, in
which artifact plays a prominent role. Check it out here:
    https://vitiral.gitbooks.io/simple-quality/content/

Seriously, its completely free and like 9 pages. You owe it to yourself to at
least skim through it -- even if you are an experienced developer and already
know this stuff.

--------------------------------------------------
## Summary and Final Words

Here are a few parting words of advice:

1. You should always write a good README and other documentation for your users
   -- design docs SHOULD be used for bringing developers of your project up
   to speed but they aren't the best format for general users.
2. Keep your design docs fairly high level -- don't try to design every detail
   using artifact. Using artifact does not mean that you shouldn't use code
   comments!
3. Use `art ls` and `art check` often, and fix those error messages!
4. follow the [artifact best practices][3]
5. Don't be afraid to refactor your design docs. It is actually easier than it
   might sound, as the tool will help you find broken links and incomplete
   items in real time. Not to mention that if you use revision control
   (you should), your artifacts can be tracked with your project -- no more
   having your documentation and your code be wildly out of sync!

This tutorial took you part of the way through developing a simple project
using artifact. Try using artifact for one of your smaller personal projects and
see the benefits that design documents can give. Have some fun with the tool,
try to break it. If you find bugs or have any suggestions, please open a ticket
at: https://github.com/vitiral/artifact/issues

Good luck!

[2]: http://wiki.openhatch.org/Flash_card_challenge
[3]: https://github.com/vitiral/artifact/blob/master/docs/BestPractices.md







# TODO: CONTINUING

## Running Your Tests

Roll back the tutorial and commit your changes.
```
art tutorial 4
git add *
git commit -m "continuing quality book tutorial"
```

Also, add `*.pyc` to your `.gitignore` file to ignore the python compiled files.

We are running the tutorial in part 4 so that there are no errors when we are
starting out (part 5 is about debugging errors).

If you followed along with the artifact interactive tutorial, you should feel
pretty confident by now that our load component is well designed and *should* be
implemented and tested. However, you haven't actually run any code yet, so you
can't be sure! We are going to change that.

The first thing you need to do is make sure you are running python2.7. Running:
```
python --version
pip --version
```

Should return something like:
```
Python 2.7.13
pip 9.0.1 from /usr/lib/python2.7/site-packages (python 2.7)
```

As long as they are both python2.7.X (but not python3.X), you are good to go.

> If not... python can be very difficult to configure.
> Search on google for how to have both python2 and python3 installed. You will
> have to do a similar exercise for `pip`.

> If it is too much of a pain, you can also just use python3 (or any other language),
> it shouldn't be difficult -- you will just have to fix any errors that come up.
>
> If you are using another language, refer to that language's unit testing guide.

Now install py.test:
```
pip install pytest
```
> you may need to use `sudo`

And run your unit tests:
```
py.test flash
```

Congratulations, you've run unit tests!

[2]: https://github.com/vitiral/artifact/blob/master/src/cmd/data/tutorial.md
