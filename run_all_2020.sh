HYPERFINE_RUN_ARGS="--warmup=10 --runs 100"

for i in $(seq -w 1 25) 
do 
    if test -f "./target/release/2020_$i"; then
        echo ""
        echo "2020 Day $i"
        ./target/release/2020_$i --input inputs/real/2020_$i
        echo ""
        hyperfine $HYPERFINE_RUN_ARGS -N -u millisecond --style basic "./target/release/2020_$i --input inputs/real/2020_$i"
    fi
done;
