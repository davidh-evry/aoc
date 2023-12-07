function get_score(hand, cards_score)
    j_count = 0
    if (cards_score[1] == 'J')
        hand = filter(c -> c != 'J', hand)
        j_count = 5 - length(hand)
    end
    map = Dict()
    for c in hand
        map[c] = get(map, c, 0) + 1
    end
    if length(map) == 1 || isempty(map)
        return 6
    elseif length(map) == 2 && any(i -> i == 1, values(map))
        return 5
    elseif length(map) == 2
        return 4
    elseif (maximum(values(map))) == 3 || any(i -> i + j_count == 3, values(map))
        return 3
    end
    pair_count = count(i -> i == 2, values(map))
    return pair_count + j_count
end

function sort_hands(h1, h2, cards_score)
    if h1[3] != h2[3]
        return h1[3] < h2[3]
    end
    for i in 1:length(h1[1])
        if h1[1][i] == h2[1][i]
            continue
        end
        h1_score = findfirst(string(h1[1][i]), cards_score)
        h2_score = findfirst(string(h2[1][i]), cards_score)
        return h1_score < h2_score
    end
    return false
end

function calculate_total(lines, cards_score)
    splits = [split(line) for line in lines]
    hands = [(hand, parse(Int, bid), get_score(hand, cards_score)) for (hand, bid) in splits]
    hands = sort(hands, lt=(h1, h2) -> sort_hands(h1, h2, cards_score))
    score = 0
    for (i, (_, bid, _)) in enumerate(hands)
        score += i * bid
    end
    println(score)
end

lines = readlines("res/day7.txt")
calculate_total(lines, "23456789TJQKA")
calculate_total(lines, "J23456789TQKA")
