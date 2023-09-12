set -e
cargo test
cargo build --release

HYPERFINE_RUN_ARGS="--warmup=10 --runs 100"

for i in $(seq -w 1 25) 
do 
    if test -f "./target/release/2020_$i"; then
        echo ""
        echo "2020 Day $i"
        ./target/release/2020_$i --input inputs/real/2020_$i
        echo ""
        # Main benchmarking
        hyperfine $HYPERFINE_RUN_ARGS -N -u millisecond --style basic "./target/release/2020_$i --input inputs/real/2020_$i" 2>/dev/null
        # CPU energy usage benchmark
        perf stat -e power/energy-pkg/ -- ./target/release/2020_$i --input inputs/real/2020_$i 2>&1 >/dev/null | grep -F "Joules"
    fi
done;
