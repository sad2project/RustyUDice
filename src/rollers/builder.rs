struct Builder(Rc<dyn ComposableRoller>)
impl Builder {
    pub fn finish(self) -> Rc<dyn ComposableRoller> { self.0 }
    pub fn n_times(self, n: i32) -> Builder { Builder(PoolRoller::basic(n, self.0)) }
    pub fn pool(self, n: i32, strategy: Strategy) -> Builder { 
        Builder(PoolRoller::new(n, self.0, strategy)) }
    pub fn with_modifier(self, relationship: Rc<dyn Relationship>, value: i32) -> Builder {
        Builder(ModiferRoller::new(self, Value { relationship, value })) }
}
impl Add<Rc<dyn ComposableRoller>> for Builder {
    type Output = Self;
    
    fn add(self, rhs: Rhs) -> Builder {
        Builder(AddRoller::new(self.0, rhs)) }
}
impl Add<Value> for Builder {
    type Output = Self;
    
    fn add(self, rhs: Rhs) -> Builder {
        Builder(ModifierRoller::new(self, rhs)) }
}
impl Subtract<Rc<dyn ComposableRoller>> for Builder {
    type Output = Self;
    
    fn subtract(self, rhs: Rhs) -> Builder {
        Builder(SubtractRoller::new(self.0, rhs)) }
}
impl Subtract<Value> for Builder {
    type Output = Self;
    
    fn subtract(self, rhs: Rhs) -> Builder {
        let value = Value{ relationship: rhs.relationship, value: rhs.value * -1 };
        Builder(ModifierRoller::new(self, value)) }
}