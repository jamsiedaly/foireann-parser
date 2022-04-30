# foireann-parser

This tool is for gaelic game's club secretaries wishing to transform Foireann CSV files into an easier to use format.

To retrieve a raw CSV navigate to [Foireann](https://www.foireann.ie/) and go to the Membership tab. Under bulk actions you should find an option to
bulk export all members. Please note, this is only possible if you have an administration account on Foireann.

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
