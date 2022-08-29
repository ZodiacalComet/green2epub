complete -c green2epub -s t -l title -d 'Title of the greentext' -r
complete -c green2epub -s a -l author -d 'Name of the author' -r
complete -c green2epub -s c -l cover -d 'Cover image to use' -r -F
complete -c green2epub -s s -l subject -l tag -d 'Greentext subjects/tags' -r
complete -c green2epub -l green-color -d 'RGB color of the green highlight in hexadecimal notation' -r
complete -c green2epub -l spoiler-color -d 'RGB color of the spoiler highlight in hexadecimal notation' -r
complete -c green2epub -l color -d 'When to use colors' -r -f -a "{auto	,always	,never	}"
complete -c green2epub -s o -l output -d 'Path for the generated epub file' -r -F
complete -c green2epub -s h -l help -d 'Print help information'
complete -c green2epub -s V -l version -d 'Print version information'
complete -c green2epub -s v -l verbose -d 'Shows verbose output, can be used multiple times to set level of verbosity'
complete -c green2epub -s q -l quiet -d 'Supress all output'
