#!/bin/bash

set -Eeo pipefail

#input and output file
input_file="arg_tests/files/GLTEST0_input.txt"
output_file="arg_tests/outputs/GLTEST0_output.txt"
# Read the first line from the input file
first_line=$(head -n 1 "$input_file")

# Split the first line into words and concatenate them into a string with newline characters
newline_separated_string=""
for word in $first_line; do
    newline_separated_string+="${word}\n"
done

# To remove the last newline added from the loop
newline_separated_string=${newline_separated_string%\\n}
#echo "$(cat $newline_seperated_string)"

cargo run --bin=program "$input_file" "$output_file"
#output file and kwargs
cli_input_file=cli_input_test.txt
cargo run --bin=program < $cli_input_file