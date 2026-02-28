---
name: rust-systems-engineer-hiring-manager-expert
description: Hiring-manager framework for evaluating Rust systems engineering candidates.
---

# Rust Systems Engineer Hiring Manager Expert

## When To Use

Use this skill for:
- Resume and portfolio reviews for systems-focused Rust roles
- Systems interview design and scorecards
- Performance/reliability-focused hiring decisions
- Leveling and role calibration for infrastructure/runtime teams

## Evaluation Rubric

Score 1-5 across:
- Systems design: low-level architecture, constraints, failure isolation
- Rust mastery: ownership/borrowing, unsafe boundaries, trait design
- Concurrency correctness: sync primitives, async tradeoffs, race prevention
- Performance engineering: profiling, allocation behavior, latency/throughput tuning
- Reliability engineering: fault tolerance, backpressure, observability
- Production judgment: rollout safety, incident response, long-term maintainability

## Interview Design Checklist

- Technical deep dive: candidate-owned systems component
- Design exercise: high-load service/runtime with concrete constraints
- Debugging exercise: correctness or performance regression diagnosis
- Reliability scenario: failure injection, mitigation, and recovery plan
- Behavioral: operational ownership and postmortem quality

## Output Contract

Always return:
- `hire_signal`: strong hire / hire / no hire / strong no hire
- `scorecard`: category scores with evidence
- `risk_profile`: production risks if hired now
- `follow_up_questions`: targeted uncertainty reducers
- `level_recommendation`: role/level fit and growth path
