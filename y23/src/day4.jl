lines = readlines("res/day4.txt")
sum_1 = 0;
copies = zeros(Int, length(lines))
for (game_num, line) in enumerate(lines)
    game = split(line, ": ")[2]
    win_part, got_part = split(game, " | ")
    win_nums = [parse(Int, x) for x in split(win_part)]
    got_nums = [parse(Int, x) for x in split(got_part)]
    matches = [x for x in got_nums if in(x, win_nums)]
    match_count = length(matches)
    if (match_count <= 0)
        continue
    end
    sum_1 += 2^(match_count - 1)
    current_card_count = copies[game_num] + 1
    for i in game_num+1:game_num+match_count
        copies[i] += current_card_count
    end
end
println(sum_1)
println(sum(values(copies)) + length(lines))
