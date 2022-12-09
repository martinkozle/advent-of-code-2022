# Generate new module for the given day
# Usage: ./generate_module.sh <day>
# Example: ./generate_module.sh 1
# Written with the help of GitHub Copilot

# check if the day is given
if [ -z "$1" ]; then
    echo "Usage: ./generate_module.sh <day>"
    exit 1
fi

# check if the day is a number
if ! [[ "$1" =~ ^[0-9]+$ ]]; then
    echo "Day must be a number"
    exit 1
fi

# check if the day is in the range
if [ "$1" -lt 1 ] || [ "$1" -gt 25 ]; then
    echo "Day must be in the range 1-25"
    exit 1
fi

# generate module only if the day is not already generated
if [ ! -d "src/days/day_$(printf "%02d" "$1")" ]; then
    # create the directory in src/days/day_n with 2 digits
    mkdir -p "src/days/day_$(printf "%02d" "$1")"

    # create the part1.rs and part2.rs files containing boilerplate.rs
    cp boilerplate.rs "src/days/day_$(printf "%02d" "$1")/part1.rs"
    cp boilerplate.rs "src/days/day_$(printf "%02d" "$1")/part2.rs"

    # create the mod.rs file containing the pub module declaration
    echo "pub mod part1;" > "src/days/day_$(printf "%02d" "$1")/mod.rs"
    echo "pub mod part2;" >> "src/days/day_$(printf "%02d" "$1")/mod.rs"

    # add the day module to the days/mod.rs file
    echo "pub mod day_$(printf "%02d" "$1");" >> src/days/mod.rs

    # add the solve function to the src/main.rs solve_function match statement as "({day}, {part}) => day_{day:02d}::part{part}::solve," above the "Add new days above this line" comment
    sed -i "s/\(.*\)\/\/ Add new days above this line/\1($1, 1) => day_$(printf "%02d" "$1")::part1::solve,\n\1($1, 2) => day_$(printf "%02d" "$1")::part2::solve,\n\1\/\/ Add new days above this line/" src/main.rs
fi


# use aoc cli to download just the input to inputs/day_{n:02d}.txt
aoc download --day "$1" --input-only --input-file "inputs/day_$(printf "%02d" "$1").txt"
