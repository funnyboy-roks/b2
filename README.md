# b2

B2 (name tbd) is a command-line tool for interacting with Backblaze's
B2.  It is similar to their B2 CLI tool written in Python, but it's
faster and more user-friendly (a.k.a. not just showing the JSON
responses from the api as the output)

## Usage

```sh
# All subcommands and arguments are visible with
b2 --help


# Authorise the user via stdin prompts
b2 authorise

# List the buckets that the user can see
b2 list-buckets

# List the files in the buckets
b2 ls
b2 ls -l

# Upload a file into b2
b2 <file> <bucket> [dest]

# Download a file from b2
b2 <file> <bucket>
b2 <file> <bucket> -O <output>
```
