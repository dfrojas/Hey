## Description

This repository it's a C extension for Python, take as base the Alice and Bob exercise from HackerRank https://www.hackerrank.com/challenges/compare-the-triplets and resolve the exercise with pure Python and with a C extension module to compare the time elapsed.

The idea behind this is NOT resolve the algorithm with the best time complexity. This repository has two main objectives:

1. Explore how to build a C extension with Python
2. Compare the time elapsed in both implementations. Take into account that both in Python and C the algorithm has the same time complexity.

## How to test it

1. Clone the repository
2. Run python3.6 setup.py build -> You will see a new build folder
3. Run python3.6 setup.py install
4. Run python3.6 alice_bob.py and see the difference

## Results

This example generates one million random integers and are passed as arguments to the functions to be proccesed and resolve the Alice and Bob exercise.

With Python the time elapsed is 4.03 seconds and with C is 0.02 seconds. Cool, right?

![alt result](https://github.com/dfrojas/exploring-c-extension-python/blob/master/result.png)

Both was executed in the same machine.
