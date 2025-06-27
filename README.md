```
  ▄▄  ▄▄▄ ▗▄▄▄▖       ▄   ▗▄▖   ▄▄ ▗▄▄▄▖▗▄ ▗▖ ▗▄▖  ▄▄▄  ▗▄▖ ▗▄ ▗▖
 █▀▀▌ ▀█▀ ▝▀█▀▘      ▐█▌ ▗▛▀▜  █▀▀▌▐▛▀▀▘▐█ ▐▌▗▛▀▜  ▀█▀  █▀█ ▐█ ▐▌
▐▌     █    █        ▐█▌ ▐▙   ▐▛   ▐▌   ▐▛▌▐▌▐▙     █  ▐▌ ▐▌▐▛▌▐▌
▐▌▗▄▖  █    █        █ █  ▜█▙ ▐▌   ▐███ ▐▌█▐▌ ▜█▙   █  ▐▌ ▐▌▐▌█▐▌
▐▌▝▜▌  █    █        ███    ▜▌▐▙   ▐▌   ▐▌▐▟▌   ▜▌  █  ▐▌ ▐▌▐▌▐▟▌
 █▄▟▌ ▄█▄   █       ▗█ █▖▐▄▄▟▘ █▄▄▌▐▙▄▄▖▐▌ █▌▐▄▄▟▘ ▄█▄  █▄█ ▐▌ █▌
  ▀▀  ▀▀▀   ▀       ▝▘ ▝▘ ▀▀▘   ▀▀ ▝▀▀▀▘▝▘ ▀▘ ▀▀▘  ▀▀▀  ▝▀▘ ▝▘ ▀▘
```
Ascend to become the 1,000,000x developer you were always destined to become! `git-ascend` is a small program that tracks your ever increasing progress every time you make a commit. 

Do you like filling bars and seeing numbers go up? Do you like writing code? You might enjoy Git Ascend.

![Screenshot From 2025-06-26 22-23-41](https://github.com/user-attachments/assets/46947df2-a4a1-43ae-bc4c-6653d9efbfec)

## Installation and setup
Download the **git-ascend** binary from the releases page and place it somewhere in your $PATH. After that, run `git ascend` and follow the instructions.

Git Ascend does not modify your git repository in any way, except for adding an (optional) post-commit hook. The hook only calls `git ascend` so you can do this manually if you don't feel like using the hook.

## Stats
There are 4 sub stats which can be leveled to multiply your main experience gain:

* **Output** increases XP gained per line of code added.
* **Pedantry** increases XP gained per line of code deleted.
* **Precision** increases XP gained based on commit message length.
* **Knowledge** increases all XP gained.

Run `git ascend switch` to change which stat you are actively leveling.

## FAQ
**What is the point of this?**

You get to watch an experience bar fill every time you make a commit. That is all.

**Lines of code/commit message length/etc are not a good measure of code quality or developer output.**

I know.
