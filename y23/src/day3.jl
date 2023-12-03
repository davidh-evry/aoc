function part1(lines)
    sum_1 = 0
    for (i, line) in enumerate(lines)
        start_i = 0
        for (j, c) in enumerate(line)
            if isdigit(c) && start_i == 0
                start_i = j
            end
            is_last_digit = j + 1 > length(line) || !isdigit(line[j+1])
            if isdigit(c) && is_last_digit && is_engine(lines, i, j)
                sum_1 += parse(Int, line[start_i:j])
                start_i = 0
            end
        end
    end
    println(sum_1)
end

function part2(lines)
    sum_2 = 0
    for (i, line) in enumerate(lines)
        for (j, c) in enumerate(line)
            if c != '*'
                continue
            end
            engine_numbers = Set{Int}()
            for x in j-1:j+1, y in i-1:i+1
                if !in_bounds(lines, y, x) || !isdigit(lines[y][x])
                    continue
                end
                end_i = find_end(lines, y, x, 1)
                if (is_engine(lines, y, end_i))
                    start_i = find_end(lines, y, x, -1)
                    push!(engine_numbers, parse(Int, lines[y][start_i:end_i]))
                end
            end
            if length(engine_numbers) == 2
                sum_2 += prod(engine_numbers)
            end
        end
    end
    println(sum_2)
end

function find_end(lines, i, j, step)
    while in_bounds(lines, i, j + step,) && isdigit(lines[i][j+step])
        j += step
    end
    return j
end

function in_bounds(lines, i, j)
    return i > 0 && i <= length(lines) && j > 0 && j <= length(lines[i])
end

function is_engine(lines, i, j)
    while j > 0 && isdigit(lines[i][j])
        for x in j-1:j+1, y in i-1:i+1
            if in_bounds(lines, y, x) && is_symbol(lines[y][x])
                return true
            end
        end
        j -= 1
    end
    return false
end

function is_symbol(c)
    return !isdigit(c) && c != '.'
end

lines = readlines("res/day3.txt")
part1(lines)
part2(lines)