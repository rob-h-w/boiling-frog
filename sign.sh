#! /usr/bin/env bash
echo wassup
shift
gpgsm --debug-level guru "$@" &> gpgsm_sign_out.txt

