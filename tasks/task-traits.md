- Create a trait called Calculator
- Sum, Sub, Mul, Divide
- Implement this on i32,f32,f64 types.

- Create a trait to implement Sum method
- trait Summing
- get(&self,)->i32
- sum($self,num:i32)->Summing


- Task-2

- Create a trait with Generic parameter as below
- implement this for two structs one is General(T), Point{l:T,b:T}
- Point does not implement 1- Add. So you need to implement std::ops::Add for Point

```rust
trait Math<T> {
    fn add(&mut self, num: T) -> &mut dyn Math<T>;
    fn get(&self) -> Self::Item;
}
```