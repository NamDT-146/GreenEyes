#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("G3ZLdixrS1yCgD7LjdLJRDb5cvieR6ujYnhdqKjsyHzf");

pub mod seahorse_util {
    use super::*;
    use std::{
        collections::HashMap,
        fmt::Debug,
        ops::{Deref, Index, IndexMut},
    };

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    pub trait IndexWrapped {
        type Output;

        fn index_wrapped(&self, index: i128) -> &Self::Output;
    }

    pub trait IndexWrappedMut: IndexWrapped {
        fn index_wrapped_mut(&mut self, index: i128) -> &mut <Self as IndexWrapped>::Output;
    }

    impl<T> IndexWrapped for Vec<T> {
        type Output = T;

        fn index_wrapped(&self, mut index: i128) -> &Self::Output {
            if index < 0 {
                index += self.len() as i128;
            }

            let index: usize = index.try_into().unwrap();

            self.index(index)
        }
    }

    impl<T> IndexWrappedMut for Vec<T> {
        fn index_wrapped_mut(&mut self, mut index: i128) -> &mut <Self as IndexWrapped>::Output {
            if index < 0 {
                index += self.len() as i128;
            }

            let index: usize = index.try_into().unwrap();

            self.index_mut(index)
        }
    }

    impl<T, const N: usize> IndexWrapped for [T; N] {
        type Output = T;

        fn index_wrapped(&self, mut index: i128) -> &Self::Output {
            if index < 0 {
                index += N as i128;
            }

            let index: usize = index.try_into().unwrap();

            self.index(index)
        }
    }

    impl<T, const N: usize> IndexWrappedMut for [T; N] {
        fn index_wrapped_mut(&mut self, mut index: i128) -> &mut <Self as IndexWrapped>::Output {
            if index < 0 {
                index += N as i128;
            }

            let index: usize = index.try_into().unwrap();

            self.index_mut(index)
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! seahorse_const {
        ($ name : ident , $ value : expr) => {
            macro_rules! $name {
                () => {
                    $value
                };
            }

            pub(crate) use $name;
        };
    }

    pub trait Loadable {
        type Loaded;

        fn load(stored: Self) -> Self::Loaded;

        fn store(loaded: Self::Loaded) -> Self;
    }

    macro_rules! Loaded {
        ($ name : ty) => {
            <$name as Loadable>::Loaded
        };
    }

    pub(crate) use Loaded;

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }

    pub(crate) use assign;

    pub(crate) use index_assign;

    pub(crate) use seahorse_const;
}

#[program]
mod mark5 {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    # [instruction (food_type_array : [u16 ; 32] , creation_time : i32 , expiration_date : i32 , harvest_time : i32 , transport_log_coordinate_lat : f64 , transport_log_coordinate_long : f64 , transport_log_time : i32 , origin_array : [u16 ; 32] , seed_random : u128)]
    pub struct CreateFoodProfile<'info> {
        #[account(mut)]
        pub payer: Signer<'info>,
        #[account(mut)]
        pub owner: Signer<'info>,
        # [account (init , space = (((((((32 + 64) + 64) + 4) + 32) + 4) + 4) + ((8 + 8) + 4)) as usize , payer = payer , seeds = [owner . key () . as_ref () , "food_profile" . as_bytes () . as_ref () , seed_random . to_le_bytes () . as_ref ()] , bump)]
        pub food_profile: Box<Account<'info, dot::program::FoodProfile>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn create_food_profile(
        ctx: Context<CreateFoodProfile>,
        food_type_array: [u16; 32],
        creation_time: i32,
        expiration_date: i32,
        harvest_time: i32,
        transport_log_coordinate_lat: f64,
        transport_log_coordinate_long: f64,
        transport_log_time: i32,
        origin_array: [u16; 32],
        seed_random: u128,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let payer = SeahorseSigner {
            account: &ctx.accounts.payer,
            programs: &programs_map,
        };

        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let food_profile = Empty {
            account: dot::program::FoodProfile::load(&mut ctx.accounts.food_profile, &programs_map),
            bump: Some(ctx.bumps.food_profile),
        };

        create_food_profile_handler(
            payer.clone(),
            owner.clone(),
            food_profile.clone(),
            food_type_array,
            creation_time,
            expiration_date,
            harvest_time,
            transport_log_coordinate_lat,
            transport_log_coordinate_long,
            transport_log_time,
            origin_array,
            seed_random,
        );

        dot::program::FoodProfile::store(food_profile.account);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct GetFoodProfile<'info> {
        #[account(mut)]
        pub food_profile: Box<Account<'info, dot::program::FoodProfile>>,
    }

    pub fn get_food_profile(ctx: Context<GetFoodProfile>) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let food_profile =
            dot::program::FoodProfile::load(&mut ctx.accounts.food_profile, &programs_map);

        get_food_profile_handler(food_profile.clone());

        dot::program::FoodProfile::store(food_profile);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (to_wallet : Pubkey)]
    pub struct TransferFood<'info> {
        #[account(mut)]
        pub from_wallet: Signer<'info>,
        #[account(mut)]
        pub food_profile: Box<Account<'info, dot::program::FoodProfile>>,
    }

    pub fn transfer_food(ctx: Context<TransferFood>, to_wallet: Pubkey) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let from_wallet = SeahorseSigner {
            account: &ctx.accounts.from_wallet,
            programs: &programs_map,
        };

        let food_profile =
            dot::program::FoodProfile::load(&mut ctx.accounts.food_profile, &programs_map);

        transfer_food_handler(from_wallet.clone(), to_wallet, food_profile.clone());

        dot::program::FoodProfile::store(food_profile);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (transport_log_coordinate_lat : f64 , transport_log_coordinate_long : f64 , transport_log_time : i32)]
    pub struct UpdateTransportLog<'info> {
        #[account(mut)]
        pub food_profile: Box<Account<'info, dot::program::FoodProfile>>,
    }

    pub fn update_transport_log(
        ctx: Context<UpdateTransportLog>,
        transport_log_coordinate_lat: f64,
        transport_log_coordinate_long: f64,
        transport_log_time: i32,
    ) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let food_profile =
            dot::program::FoodProfile::load(&mut ctx.accounts.food_profile, &programs_map);

        update_transport_log_handler(
            transport_log_coordinate_lat,
            transport_log_coordinate_long,
            transport_log_time,
            food_profile.clone(),
        );

        dot::program::FoodProfile::store(food_profile);

        return Ok(());
    }
}
