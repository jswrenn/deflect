readme:
    just generate-readme > README.md

generate-readme:
    cargo readme -t README.tpl --no-indent-headings
