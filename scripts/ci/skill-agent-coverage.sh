#!/usr/bin/env bash
set -euo pipefail

shopt -s nullglob

status=0

skill_files=(skills/*/SKILL.md)
if ((${#skill_files[@]} == 0)); then
  echo "skill-agent-coverage: no local skills found under skills/*/SKILL.md"
  exit 0
fi

for skill_file in "${skill_files[@]}"; do
  skill_dir="$(dirname "$skill_file")"
  skill_name="$(basename "$skill_dir")"
  agent_file="${skill_dir}/agents/openai.yaml"

  if [[ ! -f "$agent_file" ]]; then
    echo "skill-agent-coverage: missing agent file for '${skill_name}' (${agent_file})"
    status=1
    continue
  fi

  if ! awk '
    BEGIN {
      in_interface = 0;
      has_interface = 0;
      has_display_name = 0;
      has_short_description = 0;
      has_default_prompt = 0;
    }

    /^interface:[[:space:]]*$/ {
      has_interface = 1;
      in_interface = 1;
      next;
    }

    # Stop scanning interface block once we hit a new top-level key.
    /^[^[:space:]]/ {
      in_interface = 0;
    }

    in_interface && /^[[:space:]]+display_name:/ {
      has_display_name = 1;
    }

    in_interface && /^[[:space:]]+short_description:/ {
      has_short_description = 1;
    }

    in_interface && /^[[:space:]]+default_prompt:/ {
      has_default_prompt = 1;
    }

    END {
      exit !(has_interface && has_display_name && has_short_description && has_default_prompt);
    }
  ' "$agent_file"; then
    echo "skill-agent-coverage: '${agent_file}' must define interface.display_name, interface.short_description, and interface.default_prompt inside an 'interface:' block"
    status=1
  fi
done

if [[ -f "AGENTS.md" ]]; then
  for skill_file in "${skill_files[@]}"; do
    skill_name="$(basename "$(dirname "$skill_file")")"
    expected_line="- \`${skill_name}\` -> \`\$${skill_name}\`"

    if ! grep -Fq -- "$expected_line" AGENTS.md; then
      echo "skill-agent-coverage: AGENTS.md missing local map entry '${expected_line}'"
      status=1
    fi
  done

  while IFS= read -r mapped_skill; do
    [[ -z "$mapped_skill" ]] && continue
    if [[ ! -f "skills/${mapped_skill}/SKILL.md" ]]; then
      echo "skill-agent-coverage: AGENTS.md maps local skill '${mapped_skill}' but skills/${mapped_skill}/SKILL.md does not exist"
      status=1
    fi
  done < <(
    grep -E '^- `[a-z0-9-]+` -> `\$[a-z0-9-]+`$' AGENTS.md \
      | sed -E 's/^- `([a-z0-9-]+)` -> `\$[a-z0-9-]+`$/\1/'
  )
fi

exit "$status"
