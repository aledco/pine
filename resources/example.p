export if_example, ObjectExample

# TODO need to decide how objects should work

mod ex begin
    obj ObjectExample begin
        name: string
        value: int
    end

    fun default() -> ObjectExample begin
        return ObjectExample("default", 0)
    end

    fun inc_value(self: ObjectExample) begin
        self.value += 1
    end
end

import ex::inc_value

fun test() begin
    let o: ObjectExample = ex::default()
    ex::inc_value(o)
    o.inc_value()
end

fun if_example() begin
    let x = 0
    if x == 0 then
        print(1)
    elif x == 1 then
        print(2)
    else
        print(0)
    end

    if x > 0
    then
        print(1)
    end
end

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
