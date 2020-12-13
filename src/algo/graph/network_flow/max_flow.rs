//! An implementation of the Ford-Fulkerson (FF) method with a DFS as a method of finding augmenting
//! paths. FF allows you to find the max flow through a directed graph as well as the min cut as a
//! byproduct.
//!
//! - Time Complexity: O(fV^2), where f is the max flow
//!
//! # Resources
//!
//! - [W. Fiset's video 1](https://www.youtube.com/watch?v=LdOnanfc5TM&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=33)
//! - [W. Fiset's video 2](https://www.youtube.com/watch?v=Xu8jjJnwvxE&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=34)
//! - [W. Fiset's video 3](https://www.youtube.com/watch?v=CI5Fvk-dGVs&list=PLDV1Zeh2NRsDGO4--qE8yH72HFL1Km93P&index=32)
//! - [Wikipedia](https://www.wikiwand.com/en/Ford%E2%80%93Fulkerson_algorithm)
