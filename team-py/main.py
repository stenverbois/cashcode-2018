# Usage: python3 <algorithm> <inputfile OR 'all'>
from IO import parse, output
import sys, os
import algo1
import algo2

if __name__ == "__main__":
    if len(sys.argv) == 3:
        # Parse input file(s)
        inputs = ["input/" + sys.argv[2]]
        if sys.argv[2] == "all":
            inputs = ["input/{}".format(f) for f in os.listdir('input') if f.endswith(".in")]

        problem_datas = list(map(parse, inputs))

        solutions = list(map(globals()[sys.argv[1]].exec_alg, problem_datas))

        _ = list(map(output, inputs, solutions))

    elif len(sys.argv) == 2:
        print("ERROR: algorithm and input file required.")
