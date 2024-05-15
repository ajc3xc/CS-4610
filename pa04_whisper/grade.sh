#!/bin/bash

######## Variable globals -> ########
# The file containing the "main" entry point for the program.
# In C or C++, this is the file containing the main() function
# In Python, this is whichever file you run via python3 whatever.py
# In Bash, this is whichever file you run via bash whatever.sh
# In Rust, this is src/main.rs
main_file="main.sh"

# Any arguments you want passed to the main driver, upon excution.
# If you do not have any arguments, just leave as an empty string, ""
main_file_arguments=""

# The language that the assignment is written in.  Currently supported
# options are "cpp", "python", "bash", "rust"
language="bash"

# Whether or not to score the student using a static analyzer.
# For Python, this is the mypy type-checker.
# For C or C++, this is cppcheck.
# For Bash, this is shellcheck
# Not needed at all with rust?
enable_static_analysis=false

# Whether or not to score the student using an autoformatter (dock points
# if not formatted correctly).
# For Python, this is black.
# For C++, this is clang-format.
# For Bash, this is shfmt
# For Rust, this is rustfmt
enable_format_check=false

# Whether or not to use fuzzy or ridig diffs
# If you choose true, fuzzy diffs will give partial credit.
# This can be helpful for string-heavy assignments,
# where correctness is reasonable to estimate statistically.
# If you choose false, rigid diffs will be all-or-none.
# This is helpful when the assignment is mathy,
# where correctness is not reasonable to estimate statistically.
fuzzy_partial_credit=true

# The timeout duration in seconds for killing a student process.
# This can limit infinite run-times.
# For computationally expensive operations, you may increase this time as desired.
process_timeout=15

# The file to which the final grade is written
# This file can be saved as as artifact in gitlb CI/CD, and used to upload scores to Canvas using assigner.
student_file="results.txt"
######## <- Variable globals ########

######## File and type existence tests -> ########
# Load the specified assosicative array with files you want to check for the existence of.
# Key is the file, and Value must be a sub-string within what is produced by the bash command:
# $ file file.whatever
declare -A file_arr
file_arr=(
    ["uploads/ssh-key0.png"]="PNG image data"
    ["uploads/ssh-key1.png"]="PNG image data"
    ["uploads/iso-verify0.png"]="PNG image data"
    ["uploads/iso-verify1.png"]="PNG image data"
    ["uploads/gpg2_conversation1.png"]="PNG image data"
    ["uploads/gpg2_conversation2.png"]="PNG image data"
    ["uploads/gpg2_conversation3.png"]="PNG image data"
    ["uploads/gpg2_conversation4.png"]="PNG image data"
    ["uploads/gpg2_conversation5.png"]="PNG image data"
    ["uploads/pfs_conversation1.png"]="PNG image data"
    ["uploads/pfs_conversation2.png"]="PNG image data"
    ["uploads/pfs_conversation3.png"]="PNG image data"
    ["uploads/pfs_conversation4.png"]="PNG image data"
    ["uploads/pfs_conversation5.png"]="PNG image data"
    ["uploads/git_secure.png"]="PNG image data"
    ["uploads/id_rsa.pub"]="OpenSSH RSA public key"
    ["uploads/gpg2_key.asc"]="PGP public key block Public-Key"
)
######## <- File and type existence tests ########

######## Custom tests -> ########
# Any tests other than the unit tests and the stdout diff tests belong here,
# and must be bash functions whose names begin with "custom_test".
# Custom tests should report their score by assigning their result (0-100)
# to the custom_test_score, e.g.:
# custom_test_score=100
# Custom tests are performed alphabetically,
# so number them if you want them in order.

custom_test_0_import_key() {
    if gpg2 --import 'uploads/gpg2_key.asc' >/dev/null 2>&1; then
        custom_test_score=100
    else
        echo "GPG key imported."
        custom_test_score=0
    fi
}

custom_test_1_imported_gpg_is_4096_bit_rsa() {
    if gpg2 --list-keys $(cat uploads/email_used_for_gpg2_key.txt) | grep "rsa4096" >/dev/null 2>&1; then
        custom_test_score=100
    else
        echo "GPG key was not 4096 bit rsa."
        echo 'gpg2 --list-keys $(cat email_used_for_gpg2_key.txt) produced:'
        gpg2 --list-keys $(cat uploads/email_used_for_gpg2_key.txt)
        custom_test_score=0
    fi
}

custom_test_2_signatures() {
    # Assumes only two instances of "gpg:"
    num1=$(gpg2 --check-signatures $(cat uploads/email_used_for_gpg2_key.txt) 2>&1 | grep gpg: | head -1 | cut -d " " -f 2)
    num2=$(gpg2 --check-signatures $(cat uploads/email_used_for_gpg2_key.txt) 2>&1 | grep gpg: | tail -1 | cut -d " " -f 2)
    sum_sign=$(python3 -c "print($num1 + $num2)")
    enough=$(python3 -c "print(5 < $sum_sign)")
    if [ "$enough" == "True" ]; then
        custom_test_score=100
    else
        echo "Did not have 5 signatures other than your own (at least 6 total)."
        echo "If this test does not pass locally for you, then upload and see how it works on the server."
        custom_test_score=0
    fi
}

######## <- Custom tests ########

source .admin_files/grade_backend.sh
