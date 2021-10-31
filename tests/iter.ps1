# Put source and target colorchecker images in
# separate directories. Run: `.\iter.ps1 directoryname`
# for each directory.
#
# Make sure that the order of the images is the same
# when listed with `ls -n`. If the file order in one
# of the directories is different, the datasets will be wrong.
foreach ($x in (ls -n $args[0])) {
    .\get_colorchecker_values "$args/$x" -o "$args.txt"
}
