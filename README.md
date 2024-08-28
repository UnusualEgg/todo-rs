# CLI TODO program
## help
main:
```
$ todo -h
Usage: todo <COMMAND>

Commands:
  add     
  list    
  remove  
  folder  
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
folder:
```
$ todo folder -h
Usage: todo folder <COMMAND>

Commands:
  add          
  remove       
  list         
  set-desc     
  task         
  task-remove  
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

folder ls:
```
$ todo folder ls -h
Usage: todo folder list [OPTIONS]

Options:
  -d, --nodesc   Exclude printing descriptions of folders
  -t, --tasks    
  -h, --help     Print help
  -V, --version  Print version
```

## example
```
$ todo add "finish this README"
Added "finish this README"

$ todo add "Push this commit"
Added "Push this commit"

$ todo add "Fix borken code"
Added "Fix borken code"

$ todo folder add "github" "Things I need to do for GithHub"
Added "github"

$ todo ls
0 finish this README:
1 Push this commit:
2 Fix borken code:

$ todo folder ls -t
[0] github:
(Things I need to do for GithHub)

$ todo folder task --id 0 0
Added task to folder [0]

$ todo folder task --name "commit" 0
Added task to folder [0]

$ todo folder ls -t
[0] github:
(Things I need to do for GithHub)
	[0] [ ]finish this README
	[1] [ ]Push this commit

$ todo folder add "All Tasks" "Every single task"
Added "All Tasks"

$ todo folder task -a 1
Added task to folder [1]
Added task to folder [1]
Added task to folder [1]

$ todo folder ls -t
[0] github:
(Things I need to do for GithHub)
	[0] [ ]finish this README
	[1] [ ]Push this commit
[1] All Tasks:
(Every single task)
	[0] [ ]finish this README
	[1] [ ]Push this commit
	[2] [ ]Fix borken code

```
