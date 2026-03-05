# Visual Baselines

This directory stores baseline screenshots used by `scripts/ci/visual-snapshot.sh`.

Default paths:
- Baseline: `artifacts/visual/baseline/home.png`
- Current run: `artifacts/visual/current/home.png`

Create or refresh baseline:

```bash
VISUAL_UPDATE_BASELINE=1 scripts/ci/visual-snapshot.sh
```

Run check:

```bash
scripts/ci/visual-snapshot.sh
```
