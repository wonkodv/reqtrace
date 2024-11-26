
mkdir tests/data/${1:?test_name as 1st arg}/{,out/,expected/}

echo '
relations = []

[[artefacts]]
id = "Upper Artefact"
paths = [ "upper.md" ]
parser = "Markdown"
ignore_derived_requirements = true


[[artefacts]]
id = "Lower Artefact"
paths = [ "lower.md" ]
parser = "Markdown"

[[relations]]
upper="Upper Artefact"
lower=["Lower Artefact"]

[jobs.parse]
query="Parse"
format="Markdown"
file="out/tmx.md"

[jobs.trace]
query="Trace"
format="Markdown"
file="out/tmx.md"

[test]
jobs = ["parse", "trace", ]
success = "Success"
success = "ErrorsDetected"
out_files = [ "tmx.md" ]
covers = ["DERIVED"]

TODO
' > tests/data/$1/config.toml

echo "
The upper Artefact
==================

## REQ_1: A Requirement

## REQ_2: Another Requirement

" > tests/data/$1/upper.md

echo "
The lower Artefact
==================

## COV_1: A Coverage

## COV_2: Another Coverage

" > tests/data/$1/lower.md
