#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Define the Ticket structure with necessary fields
#[contracttype]
#[derive(Clone)]
pub struct Ticket {
    pub ticket_id: u64,
    pub event_name: String,
    pub owner: String,
    pub is_sold: bool,
    pub creation_time: u64,
}

// Define the status structure to keep track of the overall system state
#[contracttype]
#[derive(Clone)]
pub struct SystemStatus {
    pub total_tickets: u64,
    pub sold_tickets: u64,
    pub available_tickets: u64,
}

// Define mappings and symbols
const TICKET_COUNT: Symbol = symbol_short!("TIC_CONT");
const SYSTEM_STATUS: Symbol = symbol_short!("SYS_STAT");

// Enum for mapping ticket IDs
#[contracttype]
pub enum Ticketbook {
    Ticket(u64),
}

#[contract]
pub struct TicketingContract;

#[contractimpl]
impl TicketingContract {

    // Function to create a new ticket
    pub fn create_ticket(env: Env, event_name: String, owner: String) -> u64 {
        let mut ticket_count: u64 = env.storage().instance().get(&TICKET_COUNT).unwrap_or(0);
        ticket_count += 1;

        let creation_time = env.ledger().timestamp();
        
        let ticket = Ticket {
            ticket_id: ticket_count,
            event_name: event_name.clone(),
            owner: owner.clone(),
            is_sold: false,
            creation_time,
        };

        // Update the system status
        let mut status = Self::view_system_status(env.clone());
        status.total_tickets += 1;
        status.available_tickets += 1;
        env.storage().instance().set(&SYSTEM_STATUS, &status);

        // Store the ticket
        env.storage().instance().set(&Ticketbook::Ticket(ticket_count), &ticket);
        env.storage().instance().set(&TICKET_COUNT, &ticket_count);

        log!(&env, "Ticket created with ID: {}", ticket_count);

        ticket_count
    }

  pub fn transfer_ticket(env: Env, ticket_id: u64, new_owner: String) {
    let mut ticket = Self::view_ticket(env.clone(), ticket_id);

    if ticket.is_sold {
        log!(&env, "Ticket with ID {} is already sold.", ticket_id);
        panic!("Ticket already sold.");
    }

    // Clone new_owner to avoid moving the original value
    ticket.owner = new_owner.clone();
    env.storage().instance().set(&Ticketbook::Ticket(ticket_id), &ticket);

    log!(&env, "Ticket with ID {} transferred to {}", ticket_id, new_owner);
}


    // Function to mark a ticket as sold
    pub fn sell_ticket(env: Env, ticket_id: u64) {
        let mut ticket = Self::view_ticket(env.clone(), ticket_id);
        
        if ticket.is_sold {
            log!(&env, "Ticket with ID {} is already sold.", ticket_id);
            panic!("Ticket already sold.");
        }

        ticket.is_sold = true;

        // Update the system status
        let mut status = Self::view_system_status(env.clone());
        status.sold_tickets += 1;
        status.available_tickets -= 1;
        env.storage().instance().set(&SYSTEM_STATUS, &status);

        env.storage().instance().set(&Ticketbook::Ticket(ticket_id), &ticket);

        log!(&env, "Ticket with ID {} marked as sold.", ticket_id);
    }

    // Function to view ticket details
    pub fn view_ticket(env: Env, ticket_id: u64) -> Ticket {
        let key = Ticketbook::Ticket(ticket_id);
        env.storage().instance().get(&key).unwrap_or(Ticket {
            ticket_id: 0,
            event_name: String::from_str(&env, "Not Found"),
            owner: String::from_str(&env, "Not Found"),
            is_sold: false,
            creation_time: 0,
        })
    }

    // Function to view the overall system status
    pub fn view_system_status(env: Env) -> SystemStatus {
        env.storage().instance().get(&SYSTEM_STATUS).unwrap_or(SystemStatus {
            total_tickets: 0,
            sold_tickets: 0,
            available_tickets: 0,
        })
    }
}
