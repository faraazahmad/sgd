<h2>sgd</h2>

A set of commands that make terminal navigation even faster. Written in rust.

<h3>Installation</h3>

You will need:
<ul>
<li>git</li>
<li>rust and cargo</li>
</ul>

First, run the following command:

>git clone https://github.com/pknight24/sgd ~/

(NOTE: For now, you must install to the home directory. This is something that we would like to fix in the future)

Then, run:

>cargo build

and then

>cargo run --bin install

and you should be good to go. You may have to restart your terminal for changes to take effect. You can also run:

>source ~/.\*rc

(replacing \* with your shell of choice).

Also, if you use oh-my-zsh, you will need to delete or comment out the 'd' hotkey for the 'd' command to work.
Find the command here: https://github.com/robbyrussell/oh-my-zsh/wiki/Cheatsheet

<h3>Usage</h3>

Save a shortcut to the directory that you are currently in with:

>s (name)

where (name) is the name of the shortcut. For example, if I wanted to save a shortcut to a directory called 'Documents", a good shortcut name might be:

>s doc

You can also saved a specific file by running:

>s (name) -f (file)

where (file) is the path to the desired file.

Once a shortcut is saved, you can cd there automatically by simply running:

>g (name)

where, again, (name) is the name of your saved shortcut. Trying to 'g' to a shortcut that doesn't exist will throw an error. 

The 'd' command is currently still in development, but can be used if you install from the edge branch. 
It is used to run commands on files or directories saved with 's'. For example"

>d (proc) (name)

where (proc) is the name of a procedure. So far, we are having trouble implementing text editor commands with 'd', you can use more simple commands, such as
'cat' on a file or 'ls' on a directory. More work will be done on this in the future. 
