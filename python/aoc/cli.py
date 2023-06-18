def success(maybe_success):
    if maybe_success:
        return "🟢"
    return "🔴"


if __name__ == "__main__":
    from year2022 import year2022

    for dayname, day in year2022.days.items():
        print("")
        print(f"Day: {dayname}")
        if day.has_part1:
            print(f"Part 1 {success(day.test_part1_successful())}: {day.run_part1()}")
        if day.has_part2:
            print(f"Part 2 {success(day.test_part2_successful())}: {day.run_part2()}")
