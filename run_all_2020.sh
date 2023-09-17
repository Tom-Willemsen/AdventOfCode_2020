set -e

HYPERFINE_RUN_ARGS="--warmup=5 --runs 10"

for i in $(seq -w 1 25) 
do 
    if test -f "./target/release/2020_$i"; then
        CMD="./target/release/2020_$i --input inputs/real/2020_$i"
        echo ""
        echo "2020 Day $i"
        $CMD
        echo ""
        # Main benchmarking
        hyperfine $HYPERFINE_RUN_ARGS -N -u millisecond --style basic "$CMD" 2>/dev/null
        # CPU energy usage benchmark
        CPU_JOULES=$(perf stat -e power/energy-pkg/ -- $CMD 2>&1 >/dev/null | grep -oE "[0-9\.]+ Joules power/energy-pkg/" | cut -d ' ' -f 1)
        echo "CPU Joules: $CPU_JOULES J"
    fi
done;
