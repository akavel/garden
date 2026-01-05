# Right-click Send via Bluetooth on Mac

Thanks to u/Ayamykee [on r/mac](https://www.reddit.com/r/mac/comments/1b686xk/comment/mpqxvsu/):

1. `Cmd+Space` -> Automator -> create "Quick Action"
2. Set:
   - Workflow receives current: **"files or folders"**
   - in: **"Finder"**
3. Add (drag & drop) "Run Shell Script", set:
   - Shell: **`/bin/bash`**
   - Pass input: **"as arguments"**
4. Paste this:

       for f in "$@"; do
         open -a "Bluetooth File Exchange" "$f"
       done
5. File -> Save... -> **"Send via Bluetooth"**
6. Test it: Now it’ll show up in Finder > Right-click > Quick Actions
