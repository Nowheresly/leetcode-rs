use std::collections::{BTreeMap, HashMap};


#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Data {
    pub food: String,
    pub rating: i32,
}



impl PartialOrd<Self> for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rating.cmp(&other.rating) == std::cmp::Ordering::Equal {
            return Some(other.food.cmp(&self.food))
        }
        Some(self.rating.cmp(&other.rating))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct FoodRatings {
    map_food: HashMap<String, usize>,
    map_rating: HashMap<String, i32>,
    map_cuisine: HashMap<String, BTreeMap<Data, String>>,
    pub cuisines: Vec<String>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut  ret = Self {
            map_food: HashMap::new(),
            map_rating: HashMap::new(),
            map_cuisine: HashMap::new(),
            cuisines: cuisines.clone(),
        };
        for (i, food) in foods.iter().enumerate() {
            ret.map_food.insert(food.clone(), i);
            ret.map_rating.insert(food.clone(), ratings[i]);
            ret.map_cuisine.entry(cuisines[i].clone()).or_insert_with(BTreeMap::new).insert(Data { food: food.clone(), rating: ratings[i] }, food.clone());
        }

        ret
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let idx = self.map_food.get(&food).unwrap();
        let old_rating = self.map_rating.get(&food).unwrap();
        let key = Data { food: food.clone(), rating: *old_rating };
        let cuisine = self.cuisines.get(*idx).unwrap();
        self.map_cuisine.get_mut(cuisine).unwrap().remove(&key);
        self.map_cuisine.get_mut(cuisine).unwrap().insert(Data {food: food.clone(), rating: new_rating}, food.clone());
        self.map_rating.insert(food.clone(), new_rating);
    }

    fn highest_rated(&self, cuisine: String) -> String {
        self.map_cuisine.get(&cuisine).unwrap().last_key_value().unwrap().0.food.clone()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut fr = FoodRatings::new(vec![String::from("kimchi"), String::from("miso"), String::from("sushi"), String::from("moussaka"), String::from("ramen"), String::from("bulgogi")], vec![String::from("korean"), String::from("japanese"), String::from("japanese"), String::from("greek"), String::from("japanese"), String::from("korean")], vec![9, 12, 8, 15, 14, 7]);

        assert_eq!(String::from("kimchi"), fr.highest_rated(String::from("korean")));
        assert_eq!(String::from("ramen"), fr.highest_rated(String::from("japanese")));
        fr.change_rating(String::from("sushi"), 16);
        assert_eq!(String::from("sushi"), fr.highest_rated(String::from("japanese")));
        fr.change_rating(String::from("ramen"), 16);
        assert_eq!(String::from("ramen"), fr.highest_rated(String::from("japanese")));

    }
}
