
obj Cheese begin
    count: int,
    cost: float,
end

obj Milk begin
    count: int,
end

inf Product begin
    fun get_count(self) -> int
    fun set_count(self, count: int)
    fun get_cost(self) -> float
    fun set_cost(self, cost: float)
end

impl Product for Cheese begin
    fun get_count(self) -> int begin
        return self.count
    end

    fun set_count(self, count: int) begin
        set self.count = count
    end

    fun get_cost(self) -> float
    fun set_cost(self, cost: float)
end

interface
