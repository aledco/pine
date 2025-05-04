fun main() begin
    let L = private_func(1)
    for x in L
    do
        if x % 2 == 0 
        then
            print(x)
        end
    end

    return 0
end

fun if_example() begin
    let x = 0
    if x == 0 then
        print(1)
    else
        print(0)
    end

    if x > 0
    then
        print(1)
    end
end

fun while_example() begin
    let x = 0
    while x < 10 do
        print(x)
        x = x + 1
    end
end

fun ~private_func(n: int) -> [int] begin
    let L = []
    for i in 0..n do
        L.add(i)
    return L
end