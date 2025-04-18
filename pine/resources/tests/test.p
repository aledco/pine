

fun main() -> int begin
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

fun ~private_func(n: int) -> [int] begin
    L = []
    for i in 0..n do
        L.add(i)
    return L
end