#go to graph solution folder -> build exe and move to parent directory
cd ./graph_solution
cargo build --release
mv ./target/release/graph_solution.exe ../graph_solution.exe
cd ..