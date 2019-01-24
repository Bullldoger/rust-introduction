compile_rust() {
    rustc $1 -o $2
    echo "Compiled successfully: $2"
}

run_rust() {

    START=$(date +%s%N)
    
    echo "Program <$1> started"
    
    ./$1

    END=$(date +%s%N)
    DIFF=$(($END - $START))
    echo "Elapsed time : $DIFF nanoseconds"
}

clear

if [ $# -eq 2 ] 
then
    compile_rust $1 $2
    run_rust $2
elif [ $# -eq 1 ] 
then
    compile_rust $1 compiled/compiled.run
    run_rust compiled/compiled.run
else
    echo "Unresolved file with source code "
fi