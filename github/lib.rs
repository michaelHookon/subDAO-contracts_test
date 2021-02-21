#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
pub use self::github::Github;

#[ink::contract]
mod github {

    // use ink_prelude::vec::Vec; 
    // use ink_prelude::string::String;
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };

    // use ink_prelude::string;

    type GitHubId = u64;

    #[derive(scale::Encode,scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink_storage::traits::StorageLayout
        )
    )]
    pub struct PullRequest {
        repo_name: Hash,
        pr_number: u64,
        github_id: u64,
        account_id: AccountId,
    }

    #[ink(storage)]
    pub struct Github {
        length: u64,
        pullrequests: StorageHashMap<GitHubId, PullRequest>,
        auditorresults: StorageHashMap<(GitHubId, AccountId), bool>,
    }

    impl Github {
       
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                length:0,
                pullrequests: StorageHashMap::default(),
                auditorresults: StorageHashMap::default(),
             }
        }

        #[ink(message)]
        pub fn new_pull_request_auditor(& mut self, repo_name: Hash, pr_number: u64, github_id: u64, account_id: AccountId, auditor_id: AccountId) {
            let index = self.length.clone();
            self.length += 1;
            
            let pr = PullRequest{
                repo_name: repo_name,
                pr_number: pr_number,
                github_id: github_id,
                account_id: account_id,
            };
            self.pullrequests.insert(index, pr);
            self.auditorresults.insert((index,auditor_id),false);
            // self.env().emit_event(StartVote{
            //     vote_id,
            //     creator: self.env().caller(),
            // });
        }

        #[ink(message)]
        pub fn query_pull_request_auditor_status(&self, github_id: GitHubId ) -> bool{
            let mut res = false;
           for ((index, account_id),auditor_result) in &self.auditorresults {
               if index == &github_id {
                 res =  *auditor_result;
                 break;
               }
           }
           res
        }

        #[ink(message)]
        pub fn audit_pull_request(& mut self, github_id: GitHubId, audit_result: bool ) {
            let caller = self.env().caller();
            if self.auditorresults.contains_key (&(github_id,caller)) {
                self.auditorresults.insert((github_id,caller),audit_result);
            }
            
        }



        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(Default::default())
        // }

 
        // #[ink(message)]
        // pub fn flip(&mut self) {
        //     self.value = !self.value;
        // }

        // #[ink(message)]
        // pub fn get(&self) -> Vec {
        //     self.value
        // }
    }


    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// We test if the default constructor does its job.
    //     #[test]
    //     fn default_works() {
    //         let github = Github::default();
    //         assert_eq!(github.get(), false);
    //     }

    //     /// We test a simple use case of our contract.
    //     #[test]
    //     fn it_works() {
    //         let mut github = Github::new(false);
    //         assert_eq!(github.get(), false);
    //         github.flip();
    //         assert_eq!(github.get(), true);
    //     }
    // }
}