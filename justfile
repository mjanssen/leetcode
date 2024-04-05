alias c := create
alias t := test
alias r := run

create PROBLEM_NO:
    cargo generate --path ./_template --name task-{{PROBLEM_NO}}

test PROBLEM_NO:
    cargo test --package task-{{PROBLEM_NO}}

run PROBLEM_NO:
    cargo run --package task-{{PROBLEM_NO}}
