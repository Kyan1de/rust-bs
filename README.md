# rust-bs
###### name subject to change
A general-use build system written in rust

# Example(s)
```
# sets up args, to happen in sequence
set prog = "powershell"
set args = ["echo", "echo", "test"]

batch # sets up batch of commands to run together. 
    # you may not mutate a variable within a batch of commands, due to issues with race conditions.
    # "gen" generates a command to run using the given text/arguments
    gen prog *args # * unpacks the array, also indentation is optional
    gen prog *args
end # ends a batch of commands

gen prog *args # runs in sequence
gen prog *args
set args = gen prog *args # sets args to the output of the commands (in this case, ["echo", "test"])

batch # sets up the next batch of commands
    gen prog *args
end
```

# disclaimer
as you can probably tell, this is very early into development. 

todo:
 - evaluating a build once it is actually generated
 - providing examples
