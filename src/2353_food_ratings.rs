use std::collections::{HashMap, BinaryHeap};

#[derive(PartialEq, Eq)]
struct Food {
    food_rating: i32,
    food_name: String,
}

impl PartialOrd for Food {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.food_rating == other.food_rating {
            other.food_name.partial_cmp(&self.food_name)
        } else {
            self.food_rating.partial_cmp(&other.food_rating)
        }
    }
}

impl Ord for Food {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct FoodRatings {
    food_rating_map: HashMap<String, i32>,
    food_cuisine_map: HashMap<String, String>,
    cuisine_food_map: HashMap<String, BinaryHeap<Food>>,
}

// Implement some methods for the FoodRatings struct
impl FoodRatings {
    // Define a constructor that takes three vectors of foods, cuisines, and ratings
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        // Initialize the maps and the heap
        let mut food_rating_map = HashMap::new();
        let mut food_cuisine_map = HashMap::new();
        let mut cuisine_food_map = HashMap::new();

        for i in 0..foods.len() {
            food_rating_map.insert(foods[i].clone(), ratings[i]);
            food_cuisine_map.insert(foods[i].clone(), cuisines[i].clone());
            cuisine_food_map
                .entry(cuisines[i].clone())
                .or_insert_with(BinaryHeap::new)
                .push(Food {
                    food_rating: ratings[i],
                    food_name: foods[i].clone(),
                });
        }

        // Return a new instance of FoodRatings
        FoodRatings {
            food_rating_map,
            food_cuisine_map,
            cuisine_food_map,
        }
    }

    // Define a method to change the rating of a food item
    fn change_rating(&mut self, food: String, new_rating: i32) {
        // Update food's rating in 'food_rating' map
        self.food_rating_map.insert(food.to_string(), new_rating);
        // Insert the '(new rating, name)' element in the respective cuisine's priority queue
        let cuisine_name = self.food_cuisine_map.get(&food).unwrap();
        self.cuisine_food_map
            .get_mut(cuisine_name)
            .unwrap()
            .push(Food {
                food_rating: new_rating,
                food_name: food.to_string(),
            });
    }

    // Define a method to get the highest rated food item of a cuisine
    fn highest_rated(&mut self, cuisine: String) -> String {
        // Get the highest rated 'food' of 'cuisine'
        let mut highest_rated = self.cuisine_food_map.get_mut(&cuisine).unwrap().peek().unwrap().clone();

        // If the latest rating of 'food' doesn't match with the 'rating' on which it was sorted in the priority queue,
        // then we discard this element from the priority queue
        while self.food_rating_map.get(&highest_rated.food_name).unwrap() != &highest_rated.food_rating {
            self.cuisine_food_map.get_mut(&cuisine).unwrap().pop();
            highest_rated = self.cuisine_food_map.get_mut(&cuisine).unwrap().peek().unwrap().clone();
        }

        // Return the name of the highest-rated 'food' of 'cuisine'
        highest_rated.food_name.clone()
    }
}
