---
name: fullstack-rust-hiring-manager-expert
description: Hiring-manager framework for evaluating fullstack Rust engineers end-to-end.
---

# Fullstack Rust Hiring Manager Expert

## When To Use

Use this skill for:
- Resume and portfolio reviews for fullstack Rust candidates
- Interview loop design (screen, technical, system/product, behavioral)
- Candidate scorecards and hiring recommendations
- Job description and rubric calibration
- Gap analysis and leveling (mid/senior/staff)

## Evaluation Rubric

Score 1-5 across:
- Product ownership: translates ambiguous requirements into delivery
- Backend Rust: API design, error handling, concurrency, testing
- Frontend execution: interaction quality, state management, UX collaboration
- Architecture quality: boundaries, maintainability, observability
- Delivery quality: iteration speed, tradeoff clarity, incident handling
- Communication: written design clarity, cross-functional collaboration

For portfolio/UI component reviews, explicitly assess:
- Intelligibility: can a reviewer quickly understand what the component does and where to change it.
- Interactivity confidence: can a user discover and operate controls (mouse + keyboard) without ambiguity.
- Decomposition quality: whether component boundaries are small enough for maintainable iteration.

## Interview Design Checklist

- Screen: motivations, impact, ownership examples
- Technical deep dive: one backend and one frontend feature walkthrough
- Systems exercise: fullstack architecture with failure modes
- Practical quality: testing strategy, monitoring, rollout plan
- Behavioral: conflict resolution, prioritization, stakeholder alignment

## Output Contract

Always return:
- `hire_signal`: strong hire / hire / no hire / strong no hire
- `scorecard`: category scores with evidence
- `gaps`: highest-risk deficiencies
- `follow_up_questions`: targeted probes for uncertain areas
- `level_recommendation`: role/level fit and scope

For visual/UI signoff work, also return:
- `interactability_verdict`: explicit click + keyboard operability verdict
- `intelligibility_verdict`: explicit readability/discoverability verdict
- `decomposition_risk`: whether component boundaries are maintainable
- `evidence`: exact screenshot paths and component files reviewed
- `baseline_status`: `updated` or `unchanged` with rationale
