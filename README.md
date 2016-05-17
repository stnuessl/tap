# tap - task planer

## Compiling

* make
* rustc
* cargo

```
    $ make
```

## Usage

### Add tasks

```
    $ tap add [description] [time]
```

Example:

```
    $ tap add "Push something to your repository" "2022-12-31 18:00"
```
Actually you can use __"-"__"," __":"__, " ", and __"/"__ as seperators.
So running something like this

```
    $ tap add "Push something to your repository" 2022/12/31/18/00
```
is also valid. If you want to omit the exact time:
```
    $ tap add "Push something to your repository" "2022/12/31"
```

Sometimes it is easier to pass a relative time:

```
    $ tap add "Push something to your repository" 7d
```

The following characters are recognized as durations: __y__ - year, 
__m__ - months, __d__ - days, __h__ - hours, and __s__ - seconds. A complete
example would be:

```
    $ tap add "Push something to your repository" 1y2m3d4h5s
```
### Complete tasks

```
    $ tap complete [index01] [index02] ...
```

### Remove tasks

```
    $ tap remove [index01] [index02] ... [--all-completed]
    $ tap remove --all-completed
    $ tap remove --all
```