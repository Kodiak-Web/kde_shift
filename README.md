# kde_shift

simple program that sends a shift command to playerctl and displays the current default player for commands through KDE's osd system. the regex filter matches the first word in the player name - it exists because firefox.instance123456 is *not* a valid icon.

made because my sed filter broke on a zsh script that achieves the same thing, and i wanted an excuse to use rust.
this program is not very good, i'm fully aware. if i use rust again, i'm taking the time to learn how the fuck it wants error handling to be done, i tried to do it here originally but got a headache when it complained about the error not returning something specific :(
