# 🧩 Game Theory Programming Exercises

A step-by-step collection of 30+ programming challenges to learn and apply game theory concepts, from static games to advanced mechanisms and real-world applications.

## Learning Stages

### Stage 1: Foundations (Static, Simple Games)
1. **Coin Toss Guessing Game**: Simulate two players guessing heads/tails. Introduce a payoff matrix and show how player choices affect outcomes. Example: Each player picks heads or tails; if both match, Player A wins, else Player B wins.
2. **Matching Pennies**: Implement a payoff matrix for two players choosing heads or tails. Demonstrate why there’s no pure Nash equilibrium—each player tries to outguess the other.
3. **Prisoner’s Dilemma**: Simulate the classic 2×2 payoff matrix. Detect Nash equilibrium and discuss why mutual defection is stable but not optimal.
4. **Rock-Paper-Scissors**: Extend to a 3×3 game. Compute mixed strategy equilibrium and show why randomization is optimal.
5. **Battle of the Sexes**: Simulate a coordination game with two equilibria. Show how players must coordinate for best outcomes.
6. **Stag Hunt**: Implement a game of trust vs safety. Visualize the payoff landscape and discuss risk-dominant vs payoff-dominant strategies.
7. **Chicken Game**: Model escalation and brinkmanship. Simulate random vs rational strategies and discuss real-world analogies (e.g., arms race).
8. **Best Response Dynamics**: Implement an algorithm to compute best responses in a 2×2 game. Show how players adjust strategies iteratively.
9. **Payoff Matrix Parser**: Write a program that takes any matrix game and finds Nash equilibria. Example: Input a matrix, output equilibrium strategies.
10. **Brute-force Nash Finder**: Automate detection of pure/mixed equilibria for small games. Useful for validating hand-calculated results.

### Stage 2: Repeated & Stochastic Games
11. **Iterated Prisoner’s Dilemma**: Implement repeated rounds of the Prisoner’s Dilemma. Compare strategies like Always Cooperate, Always Defect, and Tit-for-Tat. Track scores over time.
12. **Strategy Tournament**: Run a round-robin tournament between different strategies. Track cumulative scores and analyze which strategy wins in the long run.
13. **Discounted Payoffs**: Introduce a discount factor for future payoffs. Measure how cooperation sustainability changes when future rewards are valued less.
14. **Noise in Iterated Games**: Add random mistakes to player actions. Test robustness of strategies like Tit-for-Tat and Grim Trigger under noisy conditions.
15. **Markov Chains for Strategies**: Represent strategies as state machines. Simulate transitions and analyze how memory affects outcomes.
16. **Evolutionary Game Simulation**: Simulate a population of strategies playing the Iterated Prisoner’s Dilemma. Weaker strategies die off, strong ones reproduce. Observe evolution over generations.
17. **Replicator Dynamics**: Implement the replicator equation to simulate evolution of strategy frequencies over time. Visualize population changes.
18. **Public Goods Game**: Model group contributions and free-riding behavior. Analyze how incentives affect cooperation in groups.
19. **Tragedy of the Commons Simulation**: Simulate multiple players exploiting a shared resource. Observe resource collapse and discuss prevention mechanisms.
20. **Minority Game**: Each player chooses an action; those in the minority win. Simulate adaptive agents and analyze emergent behavior.

### Stage 3: Advanced Mechanisms & Applications
21. **Bertrand Competition**: Simulate two firms setting prices for identical goods. Analyze equilibrium pricing and the impact of price wars.
22. **Cournot Competition**: Firms choose production quantities. Compute equilibrium and discuss how market supply affects prices.
23. **Stackelberg Competition**: Model sequential leader–follower competition. Show how the leader’s move influences the follower’s response and market outcome.
24. **Auction Simulator (First-Price, Second-Price)**: Implement Vickrey auctions. Study how different bidding strategies affect outcomes and revenue.
25. **All-Pay Auction**: Model contests where all bidders pay regardless of winning (e.g., lobbying, R&D races). Analyze incentives and expected payoffs.
26. **Matching Market (Stable Marriage)**: Implement the Gale-Shapley algorithm for stable matchings. Demonstrate how preferences lead to stable pairings.
27. **Market Clearing Prices**: Use Walrasian tatonnement to find equilibrium prices in a market. Simulate price adjustments until supply meets demand.
28. **Mechanism Design: Truthful Auction**: Implement a simple incentive-compatible mechanism (e.g., VCG auction). Show how truth-telling can be optimal.
29. **Voting Systems**: Simulate plurality, Borda count, and Condorcet voting. Demonstrate paradoxes and strategic voting scenarios.
30. **General-Sum Dynamic Game (Capstone)**: Combine repeated play, learning strategies, and equilibrium finding in a multi-agent simulation (e.g., multi-round market with strategic agents). Explore emergent behaviors and complex dynamics.

---

## 🌱 Learning Flow
- **1–10:** Internalize Nash equilibrium and best responses.
- **11–20:** Explore how time, memory, and randomness change strategies.
- **21–30:** Dive into economic game theory, mechanism design, auctions, and voting.

This progression mirrors how game theory is taught: static → repeated → stochastic → mechanism design/markets.

---

## ⚡️ Bonus Program: Optimal Stopping under Uncertainty

Simulate a scenario where signals (e.g., radio intercepts, partial decrypts) arrive over time, each with a probability of indicating an imminent attack. The challenge is to decide when to act:
- Acting too early results in a false alarm and wasted resources.
- Acting too late means the attack succeeds.

Your program should:
- Model the arrival of signals and their probabilities.
- Compute a threshold rule: “Wait until X% confidence, then act.”
- Explore sequential decision-making under uncertainty, balancing risk and reward.
- Relate the solution to Turing’s statistical decision framework and optimal stopping theory.

Example: Given a stream of signals, use Bayesian updating to estimate the probability of an attack and trigger a response when the probability exceeds a chosen threshold.