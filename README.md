# foireann-parser

A simple Rust CLI tool which parses CSV exports from Foireann and outputs the basic information needed for creating teamsheets. 

I have added the files folder to the gitignore so as to avoid accidently uploading PII to this repo as the Foireann export contains a lot of private information.

```
foireann 0.1.0

USAGE:
    foireann [OPTIONS]

OPTIONS:
    -f, --female-output-path <FEMALE_OUTPUT_PATH>    [default: files/female_members.csv]
    -h, --help                                       Print help information
    -i, --input-path <INPUT_PATH>                    [default: files/members.csv]
    -m, --male-output-path <MALE_OUTPUT_PATH>        [default: files/male_members.csv]
    -V, --version                                    Print version information
```
