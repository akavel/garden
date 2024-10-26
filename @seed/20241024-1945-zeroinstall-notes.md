# Dev-notes about 0install

Regarding a deterministic listing of apps installed on a machine
([via](https://sourceforge.net/p/zero-install/mailman/zero-install-devel/thread/CACZYt3RWbfbpH9p9icPfMoqH6o2Bo%3DVQoi7KnOtNZSpwNQbf8g%40mail.gmail.com/#msg58728255)):

```
$ ls ~/.config/0install.net/apps/
foo
...
$ 0install show --xml foo
[ includes <manifest-digest> elements ]
```
 - TODO: can I do it on Windows as well in the same way?
   - is there `0install show --xml $APP` on Windows?
     - ✖❌ nope, `0install show` prints: `Unknown command 'show'.` :(
   - is there a matching `$SOME_DIR/0install.net/apps/` directory on Windows?
  
Regarding **migrating signing keys** from a Windows machine to another one
([via](https://github.com/0install/docs/issues/26#issuecomment-2081639062)):

> The various 0install publishing tools all call `gpg` (GnuPG) under the hoods to sign feeds.
>
> On Windows GnuPG stores private keys (and other data) in `%appdata%\gnupg` by default.
> So copying that directory from your old machine to the new one should do the trick.

After installing gnupg e.g. via 0install itself, this would seem to boil down to
([via](https://unix.stackexchange.com/a/392355)):

```
On "source" machine:
$ gpg --export > all.key   # or: gpg --export $ID > single.key
$ gpg --export-secret-key > secret.key  # this or above? unsure

On "destination" machine:
$ gpg --import secret.key     # or: gpg --import all.key ? ...single.key ?
$ gpg --edit-key $ID trust quit   # needed or not needed ???
5⏎  # "I trust ultimately"
y⏎  # "Really set this key to ultimate trust?" - "Yes"
$ gpg -K    # list secret keys
$ gpg --delete-secret-key $UNUSED_KEY
```
