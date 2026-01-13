@echo off
echo See my other works at https://zephira.uk/#projects
echo Thank you for checking out xivbardarchivedownloader!
echo This small script will automatically download a giant archive of over 4 thousand MIDI files for FFXIV's Bard class.

echo Starting...
echo Do not touch the command line...

echo Downloading xivbardarchivedownloader...
curl https://vault.zephira.uk/public.php/dav/files/krcrWjq3twZ9gN7 -o xivbardarchivedownloader.exe

echo Starting xivbardarchivedownloader...
xivbardarchivedownloader.exe

echo echo Cleaning Up Midis...
setlocal EnableDelayedExpansion
set "target=downloads"
for %%f in ("%target%\*") do (
    set "name=%%~nf"
    set "ext=%%~xf"

    set "digits=!name!"
    for %%D in (0 1 2 3 4 5 6 7 8 9) do set "digits=!digits:%%D=!"

    if "!digits!"=="" (
        if /I "!ext!"==".mid" (
            del "%%f"
        )
    ) else (
        if /I not "!ext!"==".mid" (
            del "%%f"
        )
    )
)

echo Sorting Downloads...
move downloads "XIV Bard Song Archive"

echo Cleaning Up Files...
rmdir /S /Q "./logs"
del /F /A bard_ids.sqlite
del /F /A xivbardarchivedownloader.exe
del /F /A xivbardarchivedownloader.bat

echo Done.