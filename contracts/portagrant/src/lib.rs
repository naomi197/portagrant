#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod portagrant {
    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum GrantStatus {
        Active,
        Completed,
        Cancelled,
    }

    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Milestone {
        pub amount: Balance,
        pub title: String,
        pub evidence: String,
        pub submitted: bool,
        pub released: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Grant {
        pub sponsor: AccountId,
        pub builder: AccountId,
        pub total: Balance,
        pub status: GrantStatus,
        pub milestone_count: u32,
        pub released_count: u32,
    }

    #[ink(storage)]
    pub struct PortaGrant {
        next_grant_id: u64,
        grants: Mapping<u64, Grant>,
        milestones: Mapping<(u64, u32), Milestone>,
    }

    #[ink(event)]
    pub struct GrantCreated {
        #[ink(topic)]
        grant_id: u64,
        sponsor: AccountId,
        builder: AccountId,
        total: Balance,
    }

    #[ink(event)]
    pub struct EvidenceSubmitted {
        #[ink(topic)]
        grant_id: u64,
        milestone: u32,
        evidence: String,
    }

    #[ink(event)]
    pub struct MilestoneReleased {
        #[ink(topic)]
        grant_id: u64,
        milestone: u32,
        recipient: AccountId,
        amount: Balance,
    }

    impl Default for PortaGrant {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PortaGrant {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                next_grant_id: 0,
                grants: Mapping::default(),
                milestones: Mapping::default(),
            }
        }

        #[ink(message, payable)]
        pub fn create_grant(
            &mut self,
            builder: AccountId,
            titles: Vec<String>,
            amounts: Vec<Balance>,
        ) -> u64 {
            assert!(!titles.is_empty() && titles.len() == amounts.len());
            assert!(amounts.iter().all(|amount| *amount > 0));
            let total = amounts
                .iter()
                .try_fold(0u128, |sum, amount| sum.checked_add(*amount))
                .expect("grant total overflow");
            assert_eq!(self.env().transferred_value(), total);

            let grant_id = self.next_grant_id;
            self.next_grant_id = self
                .next_grant_id
                .checked_add(1)
                .expect("grant id overflow");
            let sponsor = self.env().caller();
            let milestone_count = u32::try_from(titles.len()).expect("too many milestones");

            self.grants.insert(
                grant_id,
                &Grant {
                    sponsor,
                    builder,
                    total,
                    status: GrantStatus::Active,
                    milestone_count,
                    released_count: 0,
                },
            );

            for (index, (title, amount)) in titles.into_iter().zip(amounts).enumerate() {
                let milestone = u32::try_from(index).expect("too many milestones");
                self.milestones.insert(
                    (grant_id, milestone),
                    &Milestone {
                        amount,
                        title,
                        evidence: String::new(),
                        submitted: false,
                        released: false,
                    },
                );
            }

            self.env().emit_event(GrantCreated {
                grant_id,
                sponsor,
                builder,
                total,
            });
            grant_id
        }

        #[ink(message)]
        pub fn submit_evidence(&mut self, grant_id: u64, milestone: u32, evidence: String) {
            assert!(!evidence.trim().is_empty());
            let grant = self.grants.get(grant_id).expect("grant missing");
            assert_eq!(self.env().caller(), grant.builder);
            let mut item = self
                .milestones
                .get((grant_id, milestone))
                .expect("milestone missing");
            assert!(!item.released);
            item.evidence = evidence.clone();
            item.submitted = true;
            self.milestones.insert((grant_id, milestone), &item);
            self.env().emit_event(EvidenceSubmitted {
                grant_id,
                milestone,
                evidence,
            });
        }

        #[ink(message)]
        pub fn approve_milestone(&mut self, grant_id: u64, milestone: u32) {
            let mut grant = self.grants.get(grant_id).expect("grant missing");
            assert_eq!(self.env().caller(), grant.sponsor);
            assert_eq!(grant.status, GrantStatus::Active);
            let mut item = self
                .milestones
                .get((grant_id, milestone))
                .expect("milestone missing");
            assert!(item.submitted && !item.released);

            item.released = true;
            grant.released_count = grant
                .released_count
                .checked_add(1)
                .expect("release count overflow");
            if grant.released_count == grant.milestone_count {
                grant.status = GrantStatus::Completed;
            }
            self.milestones.insert((grant_id, milestone), &item);
            self.grants.insert(grant_id, &grant);
            self.env()
                .transfer(grant.builder, item.amount)
                .expect("transfer failed");
            self.env().emit_event(MilestoneReleased {
                grant_id,
                milestone,
                recipient: grant.builder,
                amount: item.amount,
            });
        }

        #[ink(message)]
        pub fn get_grant(&self, grant_id: u64) -> Option<Grant> {
            self.grants.get(grant_id)
        }

        #[ink(message)]
        pub fn get_milestone(&self, grant_id: u64, milestone: u32) -> Option<Milestone> {
            self.milestones.get((grant_id, milestone))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        fn accounts() -> (AccountId, AccountId) {
            (AccountId::from([1; 32]), AccountId::from([2; 32]))
        }

        #[ink::test]
        fn creates_and_reads_grant() {
            let (sponsor, builder) = accounts();
            test::set_caller::<ink::env::DefaultEnvironment>(sponsor);
            test::set_value_transferred::<ink::env::DefaultEnvironment>(100);
            let mut contract = PortaGrant::new();
            let id = contract.create_grant(builder, vec![String::from("Prototype")], vec![100]);
            let grant = contract.get_grant(id).expect("grant missing");
            assert_eq!(grant.builder, builder);
            assert_eq!(grant.released_count, 0);
        }

        #[ink::test]
        fn releasing_all_milestones_completes_grant() {
            let (sponsor, builder) = accounts();
            let mut contract = PortaGrant::new();
            test::set_caller::<ink::env::DefaultEnvironment>(sponsor);
            test::set_value_transferred::<ink::env::DefaultEnvironment>(150);
            let id = contract.create_grant(
                builder,
                vec![String::from("Prototype"), String::from("Launch")],
                vec![100, 50],
            );
            test::set_caller::<ink::env::DefaultEnvironment>(builder);
            contract.submit_evidence(id, 0, String::from("ipfs://prototype"));
            contract.submit_evidence(id, 1, String::from("ipfs://launch"));
            test::set_caller::<ink::env::DefaultEnvironment>(sponsor);
            contract.approve_milestone(id, 0);
            assert_eq!(contract.get_grant(id).unwrap().status, GrantStatus::Active);
            contract.approve_milestone(id, 1);
            assert_eq!(contract.get_grant(id).unwrap().status, GrantStatus::Completed);
        }
    }
}
