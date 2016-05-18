# tap - task planer

Very simple commandline task planer I wrote to get familiar with 
[rust](https://www.rust-lang.org/).

## Installation

You will need the following tools to compile and install __tap__:

* make
* rustc
* cargo

The following commands will compile and install __tap__ on your system:

```
    $ make
    $ su -c 'make install'
```

## Usage

### Specifiy a task file

__tap__ needs a file location where it can store the tasks. You can specify
one by using:

```
    $ tap file [path]
```
Example:
```
    $ tap file /tmp/todo
```

__tap__ caches this path for consecutive invocations and can be changed 
at any point.

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
    $ tap complete --all
```

### Remove tasks

```
    $ tap remove [index01] [index02] ... [--all-completed]
    $ tap remove --all-completed
    $ tap remove --all
```
