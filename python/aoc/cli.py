from os import path
from pathlib import Path

if __name__ == "__main__":
    import year2022.day1
    from year2022 import year2022

    for dayname, day in year2022.days.items():
        print(f"Day: {dayname}")
        if day.has_part1:
            print(f"Part 1: {day.run_part1()}")
        if day.has_part2:
            print(f"Part 2: {day.run_part2()}")
