# Terra Chess

As part of Spacecamp 2021, Team Terra Chess presents....Terra Chess!

This repo is an early proof of concept for a blockchain based chess application that would form the backbone of a new community based Chess gaming protocol. 

-------------------------------

## Instructions for use

For now it's just using unit tests:
```
cargo test
```

Additional information available by
```
cargo test -- --nocapture
```

![In Progress Game](chessboard.png)

----------------------------------------------

## Background

Spend your yields to defend your Terra Chess ranking, rank up to access stronger badges!
Pool your funds with your favorite chess club, federation, or broadcaster.
Terra Chess is a decentralized banking protocol for chess arenas and their communities. Arena supporters can earn yields by providing liquidity to public arena pools, and governing the ecosystem Oracle feeds. 

[Terra Chess Gitbook](https://11chadambrose.gitbook.io/terra-chess/) 

## Road map
### **Current Functionality**
* Starting games of chess and submitting moves
* In game queries
* Match end and valid move detection
* "Playable" only via UnitTests

### **Planned Features**
* Leaderboard 
* Waiting lobby
* Staking with money and prize allocation
* Improved chess engine
* Web front end
* Gas reductions via more efficient code

### **Goals**
* Gasless interaction
* Coaching and Club deducations
* Spectator betting 

----------------------------------------------

## Contact 

We are Tim & Chad:
 - **Chad**: Overall concept, background and initial design
 - **Tim**: Smart contract design, engineering and implementation

More information can be found at: [Terra Chess Gitbook](https://11chadambrose.gitbook.io/terra-chess/) 

## Acknowledgements
- Using *kalkins' chess library* on [github](https://github.com/kalkins/rust_chess) 
- Using *CosmWasm plus* on [github](https://github.com/CosmWasm/cw-plus)

