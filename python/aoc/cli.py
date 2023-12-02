def success(maybe_success):
    if maybe_success:
        return "🟢"
    return "🔴"


def main():
    from .year2023 import year2023

    for dayname, day in year2023.days.items():
        print("")
        print(f"Day: {dayname}")
        if day.has_part1:
            print(f"Part 1 {day.test_all_part1()}: {day.run_part1()}")
        if day.has_part2:
            print(f"Part 2 {day.test_all_part2()}: {day.run_part2()}")


if __name__ == "__main__":
    main()
