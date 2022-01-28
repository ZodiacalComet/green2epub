
use builtin;
use str;

set edit:completion:arg-completer[green2epub] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'green2epub'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'green2epub'= {
            cand -t 'Title of the greentext'
            cand --title 'Title of the greentext'
            cand -a 'Name of the author'
            cand --author 'Name of the author'
            cand -c 'Cover image to use'
            cand --cover 'Cover image to use'
            cand -s 'Greentext subjects/tags'
            cand --subject 'Greentext subjects/tags'
            cand --tag 'Greentext subjects/tags'
            cand --green-color 'Color of the green highlight'
            cand --spoiler-color 'Color of the spoiler highlight'
            cand --color 'When to use colors'
            cand -o 'Path for the generated epub file'
            cand --output 'Path for the generated epub file'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand -v 'Shows verbose output, can be used multiple times to set level of verbosity'
            cand --verbose 'Shows verbose output, can be used multiple times to set level of verbosity'
            cand -q 'Supress all output'
            cand --quiet 'Supress all output'
        }
    ]
    $completions[$command]
}
