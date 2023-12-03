lines = readlines("res/day2.txt")
max_cubes = Dict("red" => 12, "green" => 13, "blue" => 14)
sum_1 = 0
sum_2 = 0
for (game_num, line) in enumerate(lines)
    start_i = findfirst(':', line)
    rounds = split(line[start_i+2:end], "; ")
    min_cubes = Dict("red" => 0, "green" => 0, "blue" => 0)
    possible = true
    for round in rounds
        cubes = split(round, ", ")
        for cube in cubes
            num_string, type = split(cube, " ")
            n = parse(Int, num_string)
            if (n > max_cubes[type])
                possible = false
            end
            if n > min_cubes[type]
                min_cubes[type] = n
            end
        end
    end
    if (possible)
        sum_1 += game_num
    end
    sum_2 += prod(values(min_cubes))
end
println(sum_1)
println(sum_2)
