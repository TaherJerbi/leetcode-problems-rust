# Desc: Generate a new problem file
# Usage: ./gen.sh <problem_name>

# Check if the problem name is given
if [ $# -eq 0 ]
  then
    echo "No problem name given"
    exit 1
fi

# Check if the problem file already exists
if [ -f "src/solution/$1.rs" ]
  then
    echo "Problem file already exists"
    exit 1
fi

# Copy the template file
cp src/template.rs src/solution/$1.rs
echo "pub mod $1;\n" >> src/solution/mod.rs

echo "Problem file created at src/solution/$1.rs and added to src/solution/mod.rs"