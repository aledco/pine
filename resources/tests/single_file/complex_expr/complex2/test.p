fun main() begin
    let x = (1 == 0 and 1 != 0) or (1 >= 1) or 400 <= -2 and 10*8 > 2**0
    if x then
        return 1
    else
        return 0
    end
end
