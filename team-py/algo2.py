def get_distance(startpos, endpos):
    return abs(startpos[0] - endpos[0]) + abs(startpos[1] - endpos[1])

def exec_alg(data):
    cars = data[0]
    # Sort on start time
    rides = sorted(data[1], key=lambda x: x.start_time)

    cars_occupied = []
    B = data[2]
    T = data[3]

    for t in range(1,T+1):

        while len(cars) > 0:
            if len(rides) == 0:
                break
            ride = rides.pop(0)
            car = cars.pop(0)
            car.set_ride(ride, t)
            cars_occupied.append(car)

        new_cars_occupied = []
        for car in cars_occupied:
            car.move()
            if car.busy_time == 0:
                cars.append(car)
            else:
                new_cars_occupied.append(car)
        cars_occupied = new_cars_occupied
        #print(cars, cars_occupied)

    return cars + cars_occupied

