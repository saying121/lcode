#!/bin/bash

time_with_color() {
    "$@"
    LEC=$?
    printf "\n"
    printf "\e[1;36m<<<<<<<<<<<<<<<<<< time >>>>>>>>>>>>>>>>>>\e[0m"
    echo -n $'\e[0;93m'
}

time time_with_color "$@"

echo -n $'\e[0m'

exit $LEC
