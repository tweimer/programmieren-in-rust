/// An evil monster.
///
/// A new monster has 100 health points. It
/// gets weaker when the health is low.
struct Monster {
    health: u8,
    strength: u8,
}


// Note: all methods and functions are
// usually defined in the same impl-block
impl Monster {
    /// Returns a monster with the specified
    /// strength.
    fn with_strength(strength: u8) -> Self {
        Self {
            health: 100,
            strength: strength,
        }
    }
    
    /// Returns a monster with strength 10. 
    fn weak() -> Self {
        Self::with_strength(10)
    }
    
    /// Returns whether or not there are
    /// any life points left.
    fn is_alive(&self) -> bool {
        self.health > 0
    }
    
    
    /// Returns the monster‘s current attack
    /// strength. If the monster has less than
    /// 20 health points, its attack is only
    /// half as strong.
    fn attack_strength(&self) -> u8 {
        if self.health < 20 {
            self.strength / 2
        } else {
            self.strength
        }
    }
    
    /// Reduces the monster’s health points
    /// according to the incoming attack’s strength.
    fn endure_attack(&mut self, strength: u8) {
        self.health = self.health.saturating_sub(strength);
    }
}

// main method
fn main() {
    let mut wolfgang = Monster::weak();
    let mut sabine = Monster::with_strength(13);
    while wolfgang.is_alive() && sabine.is_alive() {
        wolfgang.endure_attack(sabine.attack_strength());
        sabine.endure_attack(wolfgang.attack_strength());
        println!("Wolfgang: {} HP, Sabine: {} HP", wolfgang.health, sabine.health);
    }
}
