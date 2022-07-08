# Rust Smart Contract Template
# The Bus Company Management Service

## Getting started

This is a Rust smart contract that allows a transport company to maintain the records.
It has functionalities :
1. add_bus()
2. show_bus()
3. delete_bus()
4. bus_count()

## Interacting with the smart contract
<!--- Adding a bus -->
1. near call busmanagement.mashkariz_charles.testnet add_bus '{"registration_no":, "route":, "bus_capacity":, "bus_status":}' --accountId mashkariz_charles.testnet
<!--- showing a bus -->
2. near call busmanagement.mashkariz_charles.testnet show_bus --accountId mashkariz_charles.testnet
<!--- Delete bus -->
3. near call busmanagement.mashkariz_charles.testnet delete_bus --accountId mashkariz_charles.testnet
<!--- Count buses in your vector -->
4. near call busmanagement.mashkariz_charles.testnet bus_count --accountId mashkariz_charles.testnet