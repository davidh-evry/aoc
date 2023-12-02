function part1(lines)
    calibration_values = Int[]
    for line in lines
        val = line[findfirst(isdigit, line)] * line[findlast(isdigit, line)]
        push!(calibration_values, parse(Int, val))
    end
    println(sum(calibration_values))
end

function part2(lines)
    calibration_values = Int[]
    nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    for (i, line) in enumerate(lines)
        f_digit_i = findfirst(isdigit, line)
        e_digit_i = findlast(isdigit, line)
        min_i, min_num = [f_digit_i, parse(Int, line[f_digit_i])]
        max_i, max_num = [e_digit_i, parse(Int, line[e_digit_i])]
        for (num, num_string) in enumerate(nums)
            fmin_i = findfirst(num_string, line)
            fmax_i = findlast(num_string, line)
            if (fmin_i !== nothing && first(fmin_i) < min_i)
                min_i, min_num = [first(fmin_i), num]
            end
            if (fmax_i !== nothing && first(fmax_i) > max_i)
                max_i, max_num = [first(fmax_i), num]
            end
        end
        val = parse(Int, "$min_num$max_num")
        push!(calibration_values, val)
    end
    println(sum(calibration_values))
end

lines = readlines("res/day1.txt")

part1(lines)
part2(lines)
