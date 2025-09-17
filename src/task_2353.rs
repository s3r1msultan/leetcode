use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
struct Food {
    name: String,
    rating: i32,
}

impl Ord for Food {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rating
            .cmp(&other.rating)
            .then_with(|| other.name.cmp(&self.name))
    }
}

impl Eq for Food {}

impl PartialOrd for Food {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Food {
    fn eq(&self, other: &Self) -> bool {
        self.rating == other.rating && self.name == other.name
    }
}
struct FoodRatings {
    cuisine_map: HashMap<String, BinaryHeap<Food>>,
    food_map: HashMap<String, i32>,
    relation_map: HashMap<String, String>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let n = foods.len();
        let mut cuisine_map = HashMap::new();
        let mut food_map = HashMap::new();
        let mut relation_map = HashMap::new();

        for i in 0..n {
            let food = foods[i].clone();
            let cuisine = cuisines[i].clone();
            let rating = ratings[i];

            food_map.insert(food.clone(), rating);
            relation_map.insert(food.clone(), cuisine.clone());
            cuisine_map
                .entry(cuisine)
                .or_insert_with(BinaryHeap::new)
                .push(Food { name: food, rating });
        }

        Self {
            cuisine_map,
            food_map,
            relation_map,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        self.food_map.insert(food.clone(), new_rating);
        if let Some(cuisine) = self.relation_map.get(&food) {
            self.cuisine_map
                .entry(cuisine.clone())
                .or_insert_with(BinaryHeap::new)
                .push(Food {
                    rating: new_rating,
                    name: food,
                });
        }
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        if let Some(heap) = self.cuisine_map.get_mut(&cuisine) {
            while let Some(food) = heap.peek() {
                if food.rating == self.food_map.get(&food.name).unwrap().clone() {
                    return food.name.clone();
                } else {
                    heap.pop();
                }
            }
        }

        "".to_string()
    }
}
