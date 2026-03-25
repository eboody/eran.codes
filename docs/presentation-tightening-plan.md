# Presentation Tightening Plan

Last updated: March 25, 2026.

This is the active execution plan for tightening the portfolio presentation.

The site should read like one disciplined argument:

- `/` states the thesis.
- `/lab` proves it live.
- `/work/sensitive-sync` expands the flagship proof.
- `/work` is archived support.
- `/open-source` shows code-level design judgment.

Anything that does not strengthen a page's job gets removed, demoted, or collapsed.

## Execution Order

1. Fix production trust breaks first.
   Restore `https://eran.codes/work/sensitive-sync` and `https://eran.codes/resume.txt`, or remove those links from production until they work. The repo already defines those routes in code, so verify deployment output against the repo router before doing design work.

2. Freeze the reviewer path.
   Make the canonical flow `/` -> `/lab` -> `/work/sensitive-sync` -> `/open-source`. Make `/work` explicitly secondary, then align nav and CTAs to that path.

3. Simplify the global nav.
   Keep the primary nav to the minimum reviewer path. Move `Resume`, `GitHub`, `LinkedIn`, and `Contact` into quieter secondary navigation.

4. Rebuild the shared visual hierarchy.
   Reduce border treatments, badge frequency, hover lift, inset frames, and decorative gradients so the site has clear hierarchy instead of equal-emphasis surfaces.

5. Make the homepage a thesis page.
   Keep only the main claim, the current proof preview, and one compact credibility block on `/`.

6. Tighten the homepage hero.
   Keep one claim sentence, one supporting sentence, one primary CTA, and one secondary CTA. Remove non-essential badge clutter.

7. Turn `/lab` into a single flagship proof surface.
   Make the sensitive-proof panel the main event. Move burst testing, chat, and operations inspection behind explicit secondary interaction.

8. Remove explanatory duplication from the lab.
   Keep copy short and let the running UI do the proving. Delete the separate engineering-quality grid unless it adds something the proof surfaces cannot.

9. Shrink the lab to one default scroll path.
   Above the fold should show one thesis, one proof module, and one obvious action.

10. Split current proof from archive more aggressively.
    `/work/sensitive-sync` should be the full flagship narrative. `/work` should become a compact archive index, not a second long-form reading path.

11. Compress the archive layout.
    Keep one intro block for the current proof and one simple archive list below it. Do not render expanded four-card breakdowns inline for every archived case by default.

12. Keep `/open-source` as the clean code-proof page, but trim framing.
    Reduce hero aside copy and tags so the first code example becomes the focal point faster.

13. Simplify the open-source switcher interaction.
    Keep one crate selected by default and reduce control chrome before the payoff.

14. Keep auth pages minimal and give them a quieter frame.
    Reduce surrounding nav noise on auth routes so sign-in and account creation feel intentional.

15. Run a dedicated copy-tightening pass after layout changes.
    Each section gets one thesis sentence, one concrete outcome, and one CTA max. Remove repeated framing words such as `proof`, `inspect`, and `current proof surface` where they are not pulling their weight.

16. Update the visual baselines only after the IA is stable.
    Refresh the portfolio smoke images once the new hierarchy is intentional and merged.

17. Add acceptance checks that match the presentation goal.
    Guard against page sprawl, overfilled nav, and broken primary CTA targets.

## Page-Level Acceptance Criteria

- `/`: one thesis, one flagship proof preview, one credibility block.
- `/lab`: one dominant proof surface on load; secondary demos hidden until requested.
- `/work/sensitive-sync`: full current-proof story, not mixed with archive content.
- `/work`: compact archive only.
- `/open-source`: hero plus code showcase, with less explanatory framing.
- `/login` and `/register`: minimal chrome, no portfolio distraction.

## Suggested Sequence

- Week 1: fix route drift, freeze IA, simplify nav, and tune shared hierarchy primitives.
- Week 2: rebuild home and lab around one flagship proof path.
- Week 3: compress work/archive, sharpen open-source, run the copy pass, refresh baselines, and tighten tests.

## First Branch

Start with three things only:

- repair route drift for `work/sensitive-sync` and `resume.txt`
- reduce the nav to the intended reviewer path
- cut the homepage to three sections max

That gives the site a clear thesis quickly and stops later lab/work cleanup from fighting an already overstuffed shell.
