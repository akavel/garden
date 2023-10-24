# Script for capturing the lock-screen wallpaper-of-the-day on Windows

Windows 10 has a feature where its "lock screen" wallpaper can change every few days,
displaying an image selected by some people at Microsoft.
(The feature seems to be named "Spotlight".)
From time to time I find the image particularly attractive
and I want to save a copy of it for later viewing before it disappears.
I wrote the following batch script to do it.

To use the script, put it in a directory where you'd like the images to be saved,
in a file named e.g. `00-fetch.bat`
(the `00` prefix is intended to make it always stay easily visible at the top place in the directory).
Probably can be even just on desktop if you like.
Then just double-click the `00-fetch.bat` file after you notice a cool lock-screen image being shown by Windows.

```batch
@echo off
setlocal
:: https://www.groovypost.com/howto/save-windows-10-spotlight-lock-screen-pictures/
:: https://stackoverflow.com/questions/7881035/checking-file-size-in-a-batch-script

set here=%CD%
cd /d %userprofile%\AppData\Local\Packages\Microsoft.Windows.ContentDeliveryManager_*\LocalState\Assets

set minsize=200000

for %%F in (*.*) do (
    if %%~zF GTR %minsize% (
        echo %%F
        if not exist %here%\%%F.jpg (
            copy /b %%F %here%\%%F.jpg
        ) else (
            echo ...exists
        )
    )
)
cd /d %here%
pause

endlocal
```
