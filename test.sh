#!/bin/bash
assert() {
  expected="$1"
  input="$2"
  ./target/debug/compile "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

# return code only
assert 0 0
assert 42 42

# add and sub
assert 41 " 12 + 34 - 5 "

# four arithmetic operations
assert 47 '5+6*7'
assert 15 '5*(9-6)'
assert 4 '(3+5)/2'

# unary
assert 10 '-10+20'
assert 10 '9+(-1+2)'
assert 10 '- - 10'
echo OK