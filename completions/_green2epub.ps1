
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'green2epub' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'green2epub'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'green2epub' {
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Title of the greentext')
            [CompletionResult]::new('--title', 'title', [CompletionResultType]::ParameterName, 'Title of the greentext')
            [CompletionResult]::new('-a', 'a', [CompletionResultType]::ParameterName, 'Name of the author')
            [CompletionResult]::new('--author', 'author', [CompletionResultType]::ParameterName, 'Name of the author')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Cover image to use')
            [CompletionResult]::new('--cover', 'cover', [CompletionResultType]::ParameterName, 'Cover image to use')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Greentext subjects/tags')
            [CompletionResult]::new('--subject', 'subject', [CompletionResultType]::ParameterName, 'Greentext subjects/tags')
            [CompletionResult]::new('--tag', 'tag', [CompletionResultType]::ParameterName, 'Greentext subjects/tags')
            [CompletionResult]::new('--green-color', 'green-color', [CompletionResultType]::ParameterName, 'RGB color of the green highlight in hexadecimal notation')
            [CompletionResult]::new('--spoiler-color', 'spoiler-color', [CompletionResultType]::ParameterName, 'RGB color of the spoiler highlight in hexadecimal notation')
            [CompletionResult]::new('--color', 'color', [CompletionResultType]::ParameterName, 'When to use colors')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Path for the generated epub file')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'Path for the generated epub file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Shows verbose output, can be used multiple times to set level of verbosity')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows verbose output, can be used multiple times to set level of verbosity')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'Supress all output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'Supress all output')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
