# Writing Your Purpose
> **Run `art tutorial 2` to set the local directory to this stage**

A few changes have been made to your local directory:
 - the `flash_card_challenge.htm` file has been added
 - the `design/` folder has been added with [`purpose.toml`](purpose.toml)
 - [`.art/settings.toml`](settings-1.toml) has been updated with a new
   `artifact_paths`

> ### Exercise 1:
> Before we go any further, lets first explore the web-ui that you can use
> along side the command line tools. Type `art serve` and go to:
> http://127.0.0.1:5373/#artifacts/req-purpose
>
> I recommend doing most work in text files for this short interactive
> tutorial, but its good to know that the web-ui exists in case you find
> it helpful.
>
> Note that if you prefer to read this tutorial on the web, you can go to:
> https://github.com/vitiral/artifact/blob/master/src/cmd/data/tutorial.md

Open `flash_card_challenge.htm` in a browser (or go [here][2] and skim through
the project that we will be executing. Don't worry! You don't need to know
python to follow along with this tutorial.

Now open [`design/purpose.toml`](purpose.toml). This is a rough attempt to
translate the ideas in `flash_card_challenge.htm` into purpose statements.

Purpose statements are important because they document why your project even
exists -- something that is important to know as you develop it! Without
them, it can be easy to loose sight of what your project is trying to
accomplish and can be difficult to keep track of which features are
useful and which are not.

In addition, purpose statements allow you to specify what your project will
accomplish, but then complete it in pieces. **artifact** will help you track
which part is complete!

`purpose.toml` also contains the high level specifications of the program.
High-level specifications allows you to lay out your ideas for how a project
should be approached before you actually write any code. It also allows you to
write out "TODOs" that you think **should** be done, but you maybe won't get
done in your minimum viable product.

> ### Exercise 2:
> Review [`design/purpose.toml`](purpose.toml) and make sure it makes sense.
> Does this accurately summarize the application we are trying to build? Are
> there any purpose requirements missing?

Now run:
```
    art ls
```

This displays all the artifacts you just looked at, but colorizes them according
to whether they are complete or not. Right now, nothing is done so all you
see is red.

Now run:
```
    art ls SPC-cmd -l
```

This calls to list only one artifact (SPC-cmd), and displays it in the "long"
format (`-l`)

Try `art ls purpose -p` to search for all items with "purpose" in the
name, you can see that three purpose requirements appear.

> ### Exercise 3:
> Play around with the `art ls` command a little more to get used to it,
> we will be using it a lot. Get help with `art ls -h`

Once you are done, continue onto stage 3.


