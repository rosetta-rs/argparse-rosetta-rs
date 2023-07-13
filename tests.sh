#!/usr/bin/env bash


for app in examples/* ; do
    if [ $app != "examples/null-app" ] ; then
        OUT=$(cargo run -q --bin "$( basename "$app" )" -- --number 10 --no-such-flag 2> /dev/null)
        if [ $? -eq 0 ]
        then
           echo "Check if parser accepts unexpected flags"
           echo "$app produced\n$OUT"
        fi
    fi
done

for app in examples/* ; do
    if [ $app != "examples/null-app" ] ; then
        OUT=$( cargo run -q --bin "$( basename "$app" )" -- --number=10 2>&1 )
        if [ $? -ne 0 ]
        then
           echo "Check if --arg=val is accepted"
           echo "$app produced\n$OUT"
        fi
    fi
done

for app in examples/* ; do
    if [ $app != "examples/null-app" ] ; then
        OUT=$( cargo run -q --bin "$( basename "$app" )" -- -hh 2>&1 )
        if [ $? -ne 0 ]
        then
           echo "Check if -hh is accepted"
           echo "$app produced\n$OUT"
        fi
    fi
done

for app in examples/* ; do
    if [ $app != "examples/null-app" ] ; then
        OUT=$( cargo run -q --bin "$( basename "$app" )" -- --number 9000 --number 9001 2>&1 )
        if [ $? -eq 0 ]
        then
           echo "Check if duplicate flags are accepted"
           echo "$app produced\n$OUT"
        fi
    fi
done



for app in examples/* ; do
    if [ $app != "examples/null-app" ] ; then
        OUT=$( cargo run -q --bin "$( basename "$app" )" -- --number 9000000000000000000000 2>&1 )
        if [ $? -eq 0 ]
        then
           echo "Try to feed invalid numbers"
           echo "$app produced\n$OUT"
        fi
    fi
done
