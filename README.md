# Graph_Processing_Challenge-REN
Coding challenge that implements a small utility to process a directed graph represented as an adjacency list and compute several structural and algorithmic properties from it.

# Running the program
- There is a exe file created and included in the project.
- Run the exe in below command
```bash
./graph_solution path/to/graph.txt

Ex:
.\graph_solution.exe .\test_files\graph1.csv
.\graph_solution.exe .\test_files\graph2.csv
.\graph_solution.exe .\test_files\graph3.csv

```
- You can compile and build the exe file into the main directory by running the build.ps1 file as well
- Additionally, the personal notes I prepared during the assignmet are also attached as a pdf file for the reference.

NOTE: It might takes some time to process the graph and show the results (Ex: test1.csv)

# The Challange

This is a coding challenge for systems engineering candidates at Ren.

It involves implementing a small utility to process a directed graph represented as an adjacency list, and compute several structural and algorithmic properties from it.

This challenge is designed to assess your ability to:
- Parse structured input
- Build a robust data representation
- Apply basic graph algorithms
- Use a transition matrix for numerical computation
- Follow instructions carefully and write clean, testable code

## Overview

You are provided with a text file containing a directed graph in adjacency list format. Your task is to write a program that accepts the file as input and outputs the following values:

```
is_dag: true/false
max_in_degree: <integer>
max_out_degree: <integer>
pr_max: <float64>
pr_min: <float64>
```

These are described in detail in [#Output Metric Descriptions](#output-metric-descriptions).

### Executability Requirements

Your program must be runnable with a single terminal command of the form:


```bash
./graph_solution path/to/graph.txt
```

We support any language (Python, Go, Rust, etc.), but your project must include the necessary setup so that this command:

- Runs the program without needing the reviewer to manually install dependencies or compile code.
- Performs any required build/install steps automatically on first run, or instructs clearly how to prepare the executable.
- Executable on a *nix system.

**Language-Specific Notes:**
- Python / Scripting Languages:
    - Include a shebang (`#!/usr/bin/env python3`) at the top of your script.
    - Include a `requirements.txt` or similar dependency file.
    - If dependencies are required, your script should attempt to install them on first run (e.g., via `subprocess` and `pip install`).
    - If your script requires a virtual environment or custom setup, include a helper shell script like `run.sh` to handle this.
- Compiled Languages (e.g., Rust, Go, C++):
    - The `./graph_solution` command should check if a binary exists and build it automatically if not.
    - Use a Makefile, shell wrapper (`run.sh`), or build script to support this.
    - Ideally, compiled binaries should be cached so subsequent runs are fast.

**Summary**
We want to be able to:
- Clone your repo
- Run `chmod +x graph_solution` (if needed)
- Run:
```bash
./graph_solution test_cases/graph1.txt
```
…and see the output — with no additional setup.

## Input Specification

The input is a UTF-8 encoded CSV file, where each row represents a single directed edge in the graph.

Each row has exactly two values:

```csv
<source_node_id>,<target_node_id>
```

Both source_node_id and target_node_id are non-negative integers.

Each line represents a directed edge from source_node_id to target_node_id.

The file may be unordered.

Nodes that have no outgoing or incoming edges may not appear at all. Your program must handle this by computing the full set of nodes from all edge entries.

**Example**

```csv
0,1
0,2
1,3
2,3
```

This defines the following graph:

```perl
0 → 1, 2
1 → 3
2 → 3
3 → (no outgoing edges)
```

**Notes**
The graph is sparse and could contain thousands of nodes and edges.

There are no duplicate edges.

Self-edges (e.g., `5,5`) may exist and should be handled correctly.

## Output Format
Your program must output exactly five lines, one for each metric, in this format:

```txt
is_dag: true
max_in_degree: 2
max_out_degree: 2
pr_max: 0.372381
pr_min: 0.135169
```

**Notes**
- Boolean values should be true or false (lowercase).
- Degree values are integers.
- PageRank values are float64 precision, rounded to 6 decimal place

## Output Metric Descriptions

### 1. `is_dag`

Returns true if the graph is a Directed Acyclic Graph (DAG).

**Example:**

- Without a cycle

Input:

```csv
1,2
2,3
```

Output:

```txt
is_dag: true
```

- With a cycle

Input:

```csv
1,2
2,1
```
Output:

```txt
is_dag: false
```

### 2. `max_in_degree`

The maximum number of incoming edges to any single node.

**Example**

Input:

```csv
1,2
2,3
3,2
```

- `1` has 0 in-degree
- `2` has 2 in-degrees (from `1` and `3`)
- `3` has 1 in-degree (from `2`)

Output:

```txt
max_in_degree: 2
```


### 3. `max_out_degree`

The maximum number of outgoing edges from any single node.

**Example**

Input:

```csv
1,2
1,3
1,4
```

`1` has 3 outgoing edges — the max.

Output:

```txt
max_out_degree: 3
```

### 4 & 5. `pr_max` and `pr_min`

The maximum and minimum PageRank scores across all nodes.

**PageRank Specification**

You must implement the [PageRank algorithm](http://ilpubs.stanford.edu:8090/422/1/1999-66.pdf) using the following settings:

- Damping factor: d = 0.85
- Number of iterations: 20
- Handling dangling nodes: if a node has no outgoing edges, it should be treated as linking to all other nodes equally.
- The transition matrix must be row-stochastic (i.e. each row sums to 1).
- PageRank should be initialized as a uniform distribution over all nodes.
- After the 20th iteration, output the maximum and minimum PageRank scores.

**Examples**

- Without a sink

Input:

```csv
1,2
2,3
3,1
```

This forms a cycle. After running PageRank with the settings, the output would be:

```txt
pr_max: 0.333333
pr_min: 0.333333
```

- With a sink:

```csv
1,2
```

Output after 20 iterations:

```txt
pr_max: 0.649123
pr_min: 0.350877
```

Round all outputs to 6 decimal places.


## Additional Requirements

- You must include a README if special instructions are needed to run your code.
- You may use standard libraries and numerical libraries (e.g., NumPy).
- You must implement algorithms for checking if the graph is a DAG, calculating in-degrees and out-degrees.
- You **must** implement the PageRank iteration logic yourself; you **may not** use third-party PageRank implementations (e.g., from `networkx` or `scipy`).


## Provided Test Cases

In the `test_cases/` directory, you will find input and expected output files:

```txt
graph1.csv
graph1_output.txt
graph2.csv
graph2_output.txt
graph3.csv
graph3_output.txt
```

You should test your implementation against these files. We recommend verifying:

- Structural parsing
- Correctness of DAG detection
- Accurate in-degree and out-degree calculations
- Correctness in PageRank computation and matrix handling

**Expected Output**

For the test cases, a `graph<N>_output.txt` file is included. Your output must match it exactly (modulo floating point rounding).

## Tips and Tricks

- Construct your transition matrix carefully.
- Ensure your CLI accepts exactly one argument: the path to the input file.


