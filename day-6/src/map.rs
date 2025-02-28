use crate::location::Location;

#[derive(Debug, Clone)]
pub struct Map {
    pub area: [[bool; 130]; 130], // 2D array of booleans
    pub location: Location,
}



impl Map {
    // Check if current space is an obstacle
    fn obstacle_at_location(&self) -> bool {
        self.area[self.location.position.y as usize][self.location.position.x as usize]
    }
    
    // Check if location is outside of bounds of map
    fn outside_of_map(&self) -> bool {
        self.location.position.x < 0 || 
        self.location.position.x >= 130 || 
        self.location.position.y < 0 || 
        self.location.position.y >= 130
    }

    // Create an obstacle at a location
    fn create_obstacle(&mut self, location: Location) {
        self.area[location.position.y as usize][location.position.x as usize] = true;
    }
    
    // Get all locations visited by the guard
    pub fn get_locations(&self) -> Vec<Location> {
        let mut locations_visted: Vec<Location> = Vec::new();
        let mut map = self.clone();

        locations_visted.push(map.location);
        
        loop {
            map.location.step();
            
            if map.outside_of_map() {
                return locations_visted;
            }
            
            // If the guard ran into an obstacle, go back one and turn        
            if map.obstacle_at_location() {
                map.location.step_back();
                map.location.turn();
            } 
            
            locations_visted.push(map.location);
        }
    }
    
    // Check if guard is able to leave the map, given a blockage
    pub fn able_to_leave(&self, blockage: &Location) -> bool {
        let able_to_leave = true;
        let mut map = self.clone();
        map.create_obstacle(blockage.clone());


        let mut locations_visted: Vec<Location> = Vec::new();
        locations_visted.push(map.location);
        
        loop {
            map.location.step();
            
            if map.outside_of_map() {
                return able_to_leave;
            }
            
            // If the guard ran into an obstacle, go back one and turn        
            if map.obstacle_at_location() {
                map.location.step_back();
                map.location.turn();
            } 
            
            // Check if guard has returned to a location (In a loop)
            if locations_visted.contains(&map.location) {
                return !able_to_leave;
            }                 

            locations_visted.push(map.location);
        }
    }
}
    
impl Default for Map {
    fn default() -> Self {
        Self {
            area: [[false; 130]; 130], // manually initialize the 2D array
            location: Location::default()
        }
    }
}