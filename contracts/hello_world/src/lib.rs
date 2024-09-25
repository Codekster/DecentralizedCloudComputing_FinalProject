#![allow(non_snake_case)]
#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short, Map};

// Struct to store project details
#[contracttype]
#[derive(Clone)]
pub struct CloudProject {
    pub project_id: u64,
    pub title: String,       // Project title
    pub description: String, // Project description
    pub total_resources: u64, // Total resources listed
    pub active: bool,        // Is the project active?
}

// Struct to store resource listings for rent
#[contracttype]
#[derive(Clone)]
pub struct Resource {
    pub owner: String,       // Owner's account identifier
    pub resource_type: String, // Type of resource (e.g., CPU, RAM, Storage)
    pub price_per_hour: u64, // Price per hour in Lumens (XLM)
    pub available: bool,     // Is the resource available for rent?
}

// Constants for project and resource identifiers
const CLOUD_PROJECT: Symbol = symbol_short!("CLO_PRO");
const RESOURCE_COUNT: Symbol = symbol_short!("RES_COUNT");

#[contract]
pub struct CloudComputingContract;

#[contractimpl]
impl CloudComputingContract {

    // Function to create the project with title and description
    pub fn create_project(env: Env, title: String, description: String) -> u64 {
        let mut project_count: u64 = env.storage().instance().get(&CLOUD_PROJECT).unwrap_or(0);
        project_count += 1;

        let project = CloudProject {
            project_id: project_count,
            title: title.clone(),
            description: description.clone(),
            total_resources: 0,
            active: true,
        };

        env.storage().instance().set(&CLOUD_PROJECT, &project);
        
        log!(&env, "Cloud Computing Project Created: {} with ID: {}", title, project_count);
        project_count
    }

    // Function to list a resource for rent
    pub fn list_resource(env: Env, owner: String, resource_type: String, price_per_hour: u64) -> u64 {
        let mut resource_count: u64 = env.storage().instance().get(&RESOURCE_COUNT).unwrap_or(0);
        resource_count += 1;

        // Create resource listing
        let resource = Resource {
            owner: owner.clone(),
            resource_type: resource_type.clone(),
            price_per_hour: price_per_hour,
            available: true,
        };

        // Update total resource count for the project
        let mut project = Self::view_project(env.clone());
        project.total_resources += 1;

        // Store resource in a Map for flexible management
        let mut resource_map: Map<u64, Resource> = env.storage().instance().get(&Symbol::new(&env, "RESOURCES_HERE")).unwrap_or(Map::new(&env));
        resource_map.set(resource_count, resource.clone());

        env.storage().instance().set(&Symbol::new(&env, "RESOURCES"), &resource_map);
        env.storage().instance().set(&RESOURCE_COUNT, &resource_count);
        env.storage().instance().set(&CLOUD_PROJECT, &project);

        log!(&env, "Resource listed by {}: {} at {} XLM per hour", owner, resource_type, price_per_hour);

        resource_count
    }

    // Function to rent a resource
    pub fn rent_resource(env: Env, renter: String, resource_id: u64, hours: u64) -> u64 {
        let mut resource_map: Map<u64, Resource> = env.storage().instance().get(&Symbol::new(&env, "RESOURCES")).unwrap_or(Map::new(&env));

        let mut resource = resource_map.get(resource_id).unwrap_or_else(|| panic!("Resource not found"));

        if !resource.available {
            panic!("Resource is not available for rent");
        }

        let total_cost = resource.price_per_hour * hours;

        // Process payment (simulated in this contract)
        log!(&env, "Renter {} is renting {} for {} hours at a total cost of {} XLM", renter, resource.resource_type, hours, total_cost);

        // Mark the resource as unavailable
        resource.available = false;
        resource_map.set(resource_id, resource.clone());

        env.storage().instance().set(&Symbol::new(&env, "RESOURCES"), &resource_map);

        total_cost
    }

    // Function to mark a resource as available again
    pub fn release_resource(env: Env, resource_id: u64) {
        let mut resource_map: Map<u64, Resource> = env.storage().instance().get(&Symbol::new(&env, "RESOURCES")).unwrap_or(Map::new(&env));

        let mut resource = resource_map.get(resource_id).unwrap_or_else(|| panic!("Resource not found"));

        // Mark the resource as available
        resource.available = true;
        resource_map.set(resource_id, resource.clone());

        env.storage().instance().set(&Symbol::new(&env, "RESOURCES"), &resource_map);

        log!(&env, "Resource {} is now available for rent again", resource_id);
    }

    // Function to close the project
    pub fn close_project(env: Env) {
        let mut project = Self::view_project(env.clone());
        if project.active {
            project.active = false;
            env.storage().instance().set(&CLOUD_PROJECT, &project);
            log!(&env, "Cloud Computing Project closed.");
        } else {
            log!(&env, "Project is already closed!");
            panic!("Project is already closed!");
        }
    }

    // Function to view the project details
    pub fn view_project(env: Env) -> CloudProject {
        env.storage().instance().get(&CLOUD_PROJECT).unwrap_or(CloudProject {
            project_id: 0,
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
            total_resources: 0,
            active: false,
        })
    }

    // Function to view the details of a specific resource
    pub fn view_resource(env: Env, resource_id: u64) -> Resource {
        let resource_map: Map<u64, Resource> = env.storage().instance().get(&Symbol::new(&env, "RESOURCES")).unwrap_or(Map::new(&env));
        resource_map.get(resource_id).unwrap_or(Resource {
            owner: String::from_str(&env, "Not Found"),
            resource_type: String::from_str(&env, "Not Found"),
            price_per_hour: 0,
            available: false,
        })
    }
}
