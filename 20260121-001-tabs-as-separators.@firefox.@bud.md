# "New Tab" as separator in Firefox

This makes an empty "New Tab" in Firefox work as a simple visual separator
for tabs grouping.
The following tweak styles the title of all New Tabs to a heavily faded out look.

Note: You will need to tweak the "New Tab" text appropriately if you're using Firefox in a different language.

Based on:
- https://old.reddit.com/r/FirefoxCSS/wiki/index/tutorials
- https://old.reddit.com/r/firefox/comments/w3zm85/any_extension_or_workaround_to_add/

## 1. Enable `userChrome.css`

In `about:config`, search for `toolkit.legacyUserProfileCustomizations.stylesheets`
and toggle it to `true` by double-clicking it.

## 2. Find default profile directory path

Go to `about:support`, find and copy the path from _'Profile folder'_ (under _'Application Basics'_).

## 3. Create `userChrome.css`

1. Open the Profile folder path (as copied in the step above),
   create sub-folder named `chrome` (must be lowercase!).
2. In the `chrome` sub-folder,
   create file named `userChrome.css`.
3. Add the following content to the `userChrome.css` file:

       tab[label="New Tab"]:not([selected="true"]):not(:hover) {
         opacity: 0.02;
       }
       #allTabsMenu-allTabsView .all-tabs-item:has(label[value="New Tab"]):not(:hover) {
         opacity: 0.05;
       }

4. Restart Firefox (e.g. through `about:restart`).

Optionally, if you want to do some further tweaks,
the easiest way to quickly test them is with:
https://developer.mozilla.org/en-US/docs/Tools/Browser_Toolbox


