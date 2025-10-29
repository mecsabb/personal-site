+++
title = "Reinforcement Learning for Cryptocurrency Arbitrage"
date = 2023-09-01
end_date = 2024-04-01
description = "My 2023 QMIND project; building DeepMind's Alpha Zero from scratch and training it for currency arbitrage."
tags = ["Python", "Machine Learning", "Project Management"]
+++

In my 3rd year of university I led my first QMIND project. We worked on a system to apply Alpha Zero, the state-of-the-art chess and Go bot, to currency trading. The setup for this is pretty awesome, and I want to write about it in the future, but you can check out an explanation [here](https://blog.skz.dev/arbitrage-as-a-shortest-path-problem). The idea was to use the training techniques from Alpha Zero (and a graph neural network) to find shortest-paths in a graph. If you pre-process your currency graphs correctly, the shortest path may correspond to an arbitrage opportunity. *At a glance we...*

- reimplemented AlphaGo from scratch in PyTorch and trained it for 36 hrs on random graphs
- hacked together a UI to show the model processing live cryptocurrency exchange graphs
- placed top 8 at CUCAI and presented our work at the Amazon office in Toronto!