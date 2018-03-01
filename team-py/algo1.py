from itertools import count

def get_free_cars(cars):
    return [car for car in cars if car.ride is None]

def get_distance(startpos, endpos):
    return abs(startpos[0] - endpos[0]) + abs(startpos[1] - endpos[1])

def neg_to_inf(val):
    if val < 0:
        return float('-inf')
    return val



def choose_schedule(car, rides, T):
    time = 0
    rides_left = rides
    current_pos = (0,0)

    while time < T:
        print(time, T)
        if len(rides_left) == 0:
            break
        #print(car.car_id, rides_left)
        scores = get_score(current_pos, rides_left, time)
        sorted_infos = sorted(scores)

        score, ride_id = sorted_infos[0][0], sorted_infos[0][1]
        #print(ride_id)
        car.memory.append(ride_id)
        for i, ride in enumerate(rides_left):
            if ride.ride_id == ride_id:
                break

        ride = rides_left.pop(i)

        distance_to = get_distance(current_pos, ride.startpos)
        time += max(distance_to, ride.start_time - time) + ride.distance
        current_pos = ride.endpos

    return rides_left


def get_score(current_pos, rides, time):
    scores = []
    for i,ride in enumerate(rides):
        #print(ride)
        score = ride.score
        if time + max(get_distance(current_pos, ride.startpos), ride.start_time - time) + ride.distance > ride.end_time:
            scores.append((0, ride.ride_id))
        else:
            scores.append((score, ride.ride_id))

    return scores

def exec_alg(data):
    cars = data[0]
    rides = data[1]
    B = data[2]
    T = data[3]

    for car in cars:
        rides = choose_schedule(car, rides, T)

    return cars
