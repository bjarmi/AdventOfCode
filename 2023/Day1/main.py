def read_file(file_name) -> list[str]:
    with open(file_name, "r") as f:
        return f.readlines()


def main():
    lines = read_file("data.txt")

    cum_sum: int = 0
    for line in lines:
        first_num, second_num = "", ""

        for idx in range(len(line)):
            if line[idx].isnumeric():
                if first_num == "":
                    first_num = line[idx]

            if line[-(idx + 1)].isnumeric():
                if second_num == "":
                    second_num = line[-(idx + 1)]

            # Early halt
            if first_num != "" and second_num != "":
                break

        cum_sum += int(first_num + second_num)

    print(cum_sum)


if __name__ == "__main__":
    main()
