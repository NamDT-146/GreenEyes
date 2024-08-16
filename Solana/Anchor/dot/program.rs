#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct FoodProfile {
    pub owner: Pubkey,
    pub origin_array: [u16; 32],
    pub food_type_array: [u16; 32],
    pub creation_time: i32,
    pub expiration_date: i32,
    pub harvest_time: i32,
    pub transport_log_coordinate_lat: f64,
    pub transport_log_coordinate_long: f64,
    pub transport_log_time: i32,
}

impl<'info, 'entrypoint> FoodProfile {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedFoodProfile<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let origin_array =
            Mutable::new(account.origin_array.clone().map(|element| element));

        let food_type_array = Mutable::new(
            account
                .food_type_array
                .clone()
                .map(|element| element),
        );

        let creation_time = account.creation_time;
        let expiration_date = account.expiration_date;
        let harvest_time = account.harvest_time;
        let transport_log_coordinate_lat = account.transport_log_coordinate_lat;
        let transport_log_coordinate_long = account.transport_log_coordinate_long;
        let transport_log_time = account.transport_log_time;

        Mutable::new(LoadedFoodProfile {
            __account__: account,
            __programs__: programs_map,
            owner,
            origin_array,
            food_type_array,
            creation_time,
            expiration_date,
            harvest_time,
            transport_log_coordinate_lat,
            transport_log_coordinate_long,
            transport_log_time,
        })
    }

    pub fn store(loaded: Mutable<LoadedFoodProfile>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let origin_array = loaded
            .origin_array
            .clone()
            .borrow()
            .clone()
            .map(|element| element);

        loaded.__account__.origin_array = origin_array;

        let food_type_array = loaded
            .food_type_array
            .clone()
            .borrow()
            .clone()
            .map(|element| element);

        loaded.__account__.food_type_array = food_type_array;

        let creation_time = loaded.creation_time;

        loaded.__account__.creation_time = creation_time;

        let expiration_date = loaded.expiration_date;

        loaded.__account__.expiration_date = expiration_date;

        let harvest_time = loaded.harvest_time;

        loaded.__account__.harvest_time = harvest_time;

        let transport_log_coordinate_lat = loaded.transport_log_coordinate_lat;

        loaded.__account__.transport_log_coordinate_lat = transport_log_coordinate_lat;

        let transport_log_coordinate_long = loaded.transport_log_coordinate_long;

        loaded.__account__.transport_log_coordinate_long = transport_log_coordinate_long;

        let transport_log_time = loaded.transport_log_time;

        loaded.__account__.transport_log_time = transport_log_time;
    }
}

#[derive(Debug)]
pub struct LoadedFoodProfile<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, FoodProfile>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub origin_array: Mutable<[u16; 32]>,
    pub food_type_array: Mutable<[u16; 32]>,
    pub creation_time: i32,
    pub expiration_date: i32,
    pub harvest_time: i32,
    pub transport_log_coordinate_lat: f64,
    pub transport_log_coordinate_long: f64,
    pub transport_log_time: i32,
}

pub fn create_food_profile_handler<'info>(
    mut payer: SeahorseSigner<'info, '_>,
    mut owner: SeahorseSigner<'info, '_>,
    mut food_profile: Empty<Mutable<LoadedFoodProfile<'info, '_>>>,
    mut food_type_array: [u16; 32],
    mut creation_time: i32,
    mut expiration_date: i32,
    mut harvest_time: i32,
    mut transport_log_coordinate_lat: f64,
    mut transport_log_coordinate_long: f64,
    mut transport_log_time: i32,
    mut origin_array: [u16; 32],
    mut seed_random: u128,
) -> () {
    let mut food_profile = food_profile.account.clone();

    assign!(food_profile.borrow_mut().owner, owner.key());

    assign!(food_profile.borrow_mut().origin_array, Mutable::<[u16; 32]>::new(origin_array));

    assign!(food_profile.borrow_mut().food_type_array, Mutable::<[u16; 32]>::new(food_type_array));

    assign!(food_profile.borrow_mut().creation_time, creation_time);

    assign!(food_profile.borrow_mut().expiration_date, expiration_date);

    assign!(food_profile.borrow_mut().harvest_time, harvest_time);

    assign!(
        food_profile.borrow_mut().transport_log_coordinate_lat,
        transport_log_coordinate_lat
    );

    assign!(
        food_profile.borrow_mut().transport_log_coordinate_long,
        transport_log_coordinate_long
    );

    assign!(
        food_profile.borrow_mut().transport_log_time,
        transport_log_time
    );

    solana_program::msg!(
        "{}",
        format!(
            "Food Profile created at {:?}",
            food_profile.borrow().__account__.key()
        )
    );
}

pub fn get_food_profile_handler<'info>(
    mut food_profile: Mutable<LoadedFoodProfile<'info, '_>>,
) -> () {
    solana_program::msg!("{}", format!("Owner: {:?}", food_profile.borrow().owner));

    solana_program::msg!(
        "{}",
        format!(
            "Food Type Array: {:?}",
            food_profile.borrow().food_type_array
        )
    );

    solana_program::msg!(
        "{}",
        format!("Origin: {:?}", food_profile.borrow().origin_array)
    );

    solana_program::msg!(
        "{}",
        format!("Creation Time: {}", food_profile.borrow().creation_time)
    );

    solana_program::msg!(
        "{}",
        format!("Expiration Date: {}", food_profile.borrow().expiration_date)
    );

    solana_program::msg!(
        "{}",
        format!("Harvest Time: {}", food_profile.borrow().harvest_time)
    );

    solana_program::msg!(
        "{}",
        format!(
            "Transport Log Latitude: {}",
            food_profile.borrow().transport_log_coordinate_lat
        )
    );

    solana_program::msg!(
        "{}",
        format!(
            "Transport Log Longitude: {}",
            food_profile.borrow().transport_log_coordinate_long
        )
    );

    solana_program::msg!(
        "{}",
        format!(
            "Transport Log Time: {}",
            food_profile.borrow().transport_log_time
        )
    );
}

pub fn transfer_food_handler<'info>(
    mut from_wallet: SeahorseSigner<'info, '_>,
    mut to_wallet: Pubkey,
    mut food_profile: Mutable<LoadedFoodProfile<'info, '_>>,
) -> () {
    if !(from_wallet.key() == food_profile.borrow().owner) {
        panic!("Only the owner can transfer this food profile.");
    }

    assign!(food_profile.borrow_mut().owner, to_wallet);

    solana_program::msg!(
        "{}",
        format!(
            "Food Profile {:?} transferred from {:?} to {:?}",
            food_profile.borrow().__account__.key(),
            from_wallet.key(),
            to_wallet
        )
    );
}

pub fn update_transport_log_handler<'info>(
    mut transport_log_coordinate_lat: f64,
    mut transport_log_coordinate_long: f64,
    mut transport_log_time: i32,
    mut food_profile: Mutable<LoadedFoodProfile<'info, '_>>,
) -> () {
    assign!(
        food_profile.borrow_mut().transport_log_coordinate_lat,
        transport_log_coordinate_lat
    );

    assign!(
        food_profile.borrow_mut().transport_log_coordinate_long,
        transport_log_coordinate_long
    );

    assign!(
        food_profile.borrow_mut().transport_log_time,
        transport_log_time
    );

    solana_program::msg!(
        "{}",
        format!(
            "Updated transport log to latitude: {}, longitude: {}, time: {}",
            transport_log_coordinate_lat, transport_log_coordinate_long, transport_log_time
        )
    );
}
