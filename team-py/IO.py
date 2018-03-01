from ClassFile import *

# Reads file with name filename and outputs dictionary
def parse(filename):
    with open(filename, mode="r") as ifs:
        lines = ifs.readlines()
        dictionary = {"filename": filename[6:]}

        # Line 1 (VERCX)
        [V, E, R, C, X] = map(int, lines[0].split())
        dictionary['V'] = V
        dictionary['E'] = E
        dictionary['R'] = R
        dictionary['C'] = C
        dictionary['X'] = X

        # Line 2 (video sizes)
        dictionary['S']= list(map(int, lines[1].split()))

        # Latencies
        line = 2
        endpoint = 0
        dictionary['Ld'] = [0 for _ in range(E)]
        dictionary['Lc'] = {}
        while endpoint < E:
            [L, K] = map(int, lines[line].split())
            dictionary['Ld'][endpoint] = L
            for j in range(K):
                [cacheId, latency] = map(int, lines[line+j+1].split())
                dictionary['Lc'][(endpoint,cacheId)] = latency
            endpoint += 1
            line += K+1

        # Requests
        def parseReqLine(line):
            [v, e, r] = map(int, line.split())
            return (v, e, r)
        dictionary['Rqs'] = list(map(parseReqLine, lines[line:]))

        # Return problem data
        return dictionary

# Output the solution
def output(filename, solution):
    # Cut off 'input/'
    with open('output_{}.txt'.format(filename[6:-3]), mode="w") as ofs:
        print("Outputting solution for {}...".format(filename[6:]))
        if solution and isinstance(solution[0], ?):
            pass
        else:
            ofs.write("{}\n".format(len(solution)))
            for idx, s in enumerate(solution):
                ofs.write("{} {}\n".format(idx, " ".join(map(str, s))))
