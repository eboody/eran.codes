#!/usr/bin/env bash
set -euo pipefail

roots=()
for candidate in crates src tests; do
  if [[ -d "$candidate" ]]; then
    roots+=("$candidate")
  fi
done

if ((${#roots[@]} == 0)); then
  exit 0
fi

if ! command -v rg >/dev/null 2>&1; then
  if ! command -v grep >/dev/null 2>&1; then
    echo "error: rg or grep is required for this check."
    exit 1
  fi
fi

collect_marked_modules() {
  if command -v rg >/dev/null 2>&1; then
    rg --no-heading --line-number -g '*.rs' \
      '^\s*//\s*ci:\s*descriptive-module-import\s+[A-Za-z0-9_:]+' \
      "${roots[@]}" \
      | sed -E 's#^([^:]+):[0-9]+:.*descriptive-module-import[[:space:]]+([A-Za-z0-9_:]+).*#\1|\2#' \
      | sort -u
  else
    grep -RnoE --include='*.rs' \
      '^[[:space:]]*//[[:space:]]*ci:[[:space:]]*descriptive-module-import[[:space:]]+[A-Za-z0-9_:]+' \
      "${roots[@]}" \
      | sed -E 's#^([^:]+):[0-9]+:.*descriptive-module-import[[:space:]]+([A-Za-z0-9_:]+).*#\1|\2#' \
      | sort -u
  fi
}

check_module_leaf_imports() {
  local module="$1"
  local parent="${module%::*}"
  local leaf="${module##*::}"
  local direct_pattern="^\\s*use\\s+${module}::"
  local brace_pattern="^\\s*use\\s+${parent}::\\{[^;]*([,{[:space:]])${leaf}::"

  if command -v rg >/dev/null 2>&1; then
    rg --no-heading --line-number -g '*.rs' \
      -e "$direct_pattern" \
      -e "$brace_pattern" \
      "${roots[@]}" || true
  else
    grep -RnoE --include='*.rs' \
      -e "$direct_pattern" \
      -e "$brace_pattern" \
      "${roots[@]}" || true
  fi
}

check_module_internal_surface_imports() {
  local marker_file="$1"
  local module="$2"
  local module_leaf="${module##*::}"
  local marker_dir
  marker_dir="$(dirname "$marker_file")"

  if [[ "$(basename "$marker_dir")" != "$module_leaf" ]]; then
    return
  fi

  local super_direct_pattern='^\s*use\s+super::[a-z_][A-Za-z0-9_]*::'
  local super_brace_pattern='^\s*use\s+super::\{[^;]*([,{[:space:]])[a-z_][A-Za-z0-9_]*::'

  if command -v rg >/dev/null 2>&1; then
    rg --no-heading --line-number -g '*.rs' \
      -e "$super_direct_pattern" \
      -e "$super_brace_pattern" \
      "$marker_dir" || true
  else
    grep -RnoE --include='*.rs' \
      -e "$super_direct_pattern" \
      -e "$super_brace_pattern" \
      "$marker_dir" || true
  fi
}

extract_parent_type_names() {
  local marker_file="$1"
  perl -0777 -ne '
    my %seen = ();
    while (/pub\s+use\s+([^;]+);/sg) {
      my $stmt = $1;
      my @items;
      if ($stmt =~ /\{(.*)\}/s) {
        @items = split /,/, $1;
      } else {
        @items = ($stmt);
      }
      for my $item (@items) {
        $item =~ s/^\s+|\s+$//g;
        next if $item eq q{};
        if ($item =~ /\bas\s+([A-Z][A-Za-z0-9_]*)\b/) {
          $seen{$1} = 1;
          next;
        }
        if ($item =~ /([A-Z][A-Za-z0-9_]*)\s*$/) {
          $seen{$1} = 1;
        }
      }
    }
    while (/pub\s+(?:struct|enum)\s+([A-Z][A-Za-z0-9_]*)/sg) {
      $seen{$1} = 1;
    }
    for my $name (sort keys %seen) {
      print "$name\n";
    }
  ' "$marker_file"
}

prefix_of_type_name() {
  local type_name="$1"
  perl -e '
    my $name = shift;
    my @chars = split //, $name;
    my $out = $chars[0] // q{};
    for (my $i = 1; $i < scalar(@chars); $i++) {
      my $c = $chars[$i];
      my $prev = $chars[$i - 1] // q{};
      my $next = $chars[$i + 1] // q{};
      if ($c =~ /[A-Z]/ && (($next =~ /[a-z]/) || ($prev =~ /[a-z]/))) {
        last;
      }
      $out .= $c;
    }
    print $out;
  ' "$type_name"
}

check_family_prefix_breakdown() {
  local marker_file="$1"
  local module="$2"
  local module_leaf="${module##*::}"
  local marker_dir
  marker_dir="$(dirname "$marker_file")"

  # Only apply the family rule inside the module's own exposing file.
  if [[ "$(basename "$marker_dir")" != "$module_leaf" ]]; then
    return
  fi

  mapfile -t type_names < <(extract_parent_type_names "$marker_file")
  if ((${#type_names[@]} == 0)); then
    return
  fi

  # Build family -> names map by first Pascal segment.
  declare -A family_count=()
  declare -A family_names=()

  for type_name in "${type_names[@]}"; do
    local family
    family="$(prefix_of_type_name "$type_name")"
    if [[ -z "$family" ]]; then
      continue
    fi
    family_count["$family"]=$(( ${family_count["$family"]:-0} + 1 ))
    if [[ -n "${family_names[$family]:-}" ]]; then
      family_names["$family"]="${family_names[$family]} $type_name"
    else
      family_names["$family"]="$type_name"
    fi
  done

  for family in "${!family_count[@]}"; do
    local count="${family_count[$family]}"
    if ((count <= 1)); then
      continue
    fi

    local family_module
    family_module="$(printf '%s' "$family" | tr '[:upper:]' '[:lower:]')"
    local has_family_module=0
    if grep -Eq "(^|[^A-Za-z0-9_])mod[[:space:]]+${family_module}([[:space:];,}])" "$marker_file"; then
      has_family_module=1
    fi

    local names="${family_names[$family]}"
    local canonical_count=0
    for n in $names; do
      if [[ "$n" == "$family" ]]; then
        canonical_count=$((canonical_count + 1))
      fi
    done

    if ((has_family_module == 0)); then
      echo "${marker_file}:1: multiple ${family}* symbols in parent surface (${names})."
      echo "error: '${module}' should scope '${family}*' into '${module_leaf}::${family_module}::*' when more than one exists."
      echo
      fail=1
      continue
    fi

    local non_canonical_count=$((count - canonical_count))
    if ((non_canonical_count > 0)); then
      echo "${marker_file}:1: parent surface still exports multiple ${family}* symbols (${names})."
      echo "error: keep at most canonical '${family}' at parent (if needed); move companions to '${module_leaf}::${family_module}::*'."
      echo
      fail=1
    fi
  done
}

check_compound_sibling_modules() {
  local marker_file="$1"
  local module="$2"
  local module_leaf="${module##*::}"
  local marker_dir
  marker_dir="$(dirname "$marker_file")"

  # Only apply when marker lives in the module directory itself (for example .../chat/mod.rs).
  if [[ "$(basename "$marker_dir")" != "$module_leaf" ]]; then
    return
  fi

  local parent_dir
  parent_dir="$(dirname "$marker_dir")"
  local hits=()

  while IFS= read -r file; do
    hits+=("$file")
  done < <(find "$parent_dir" -maxdepth 1 -type f -name "${module_leaf}_*.rs" | sort)

  if ((${#hits[@]} <= 1)); then
    return
  fi

  for hit in "${hits[@]}"; do
    local base
    base="$(basename "$hit" .rs)"
    local suffix="${base#${module_leaf}_}"
    echo "${hit}:1: compound sibling module file for descriptive module '${module_leaf}'."
    echo "error: move '${base}.rs' under '${module_leaf}/${suffix}.rs' and expose it through '${module_leaf}::${suffix}'."
    echo
  done
  echo "note: this rule triggers only when 2+ sibling files share the '${module_leaf}_*' prefix."
  echo
  fail=1
}

check_general_compound_prefix_groups() {
  local dir
  while IFS= read -r dir; do
    declare -A prefix_counts=()
    declare -A prefix_hits=()

    while IFS= read -r hit; do
      local base
      base="$(basename "$hit" .rs)"

      # Files that explicitly declare descriptive module import policy are
      # already namespaced surfaces; skip generic sibling-prefix heuristics.
      if grep -Eq '^[[:space:]]*//[[:space:]]*ci:[[:space:]]*descriptive-module-import[[:space:]]+[A-Za-z0-9_:]+' "$hit"; then
        continue
      fi

      if [[ "$base" != *_* ]]; then
        continue
      fi

      local prefix
      prefix="${base%%_*}"
      prefix_counts["$prefix"]=$(( ${prefix_counts["$prefix"]:-0} + 1 ))
      if [[ -n "${prefix_hits[$prefix]:-}" ]]; then
        prefix_hits["$prefix"]+=$'\n'"$hit"
      else
        prefix_hits["$prefix"]="$hit"
      fi
    done < <(find "$dir" -maxdepth 1 -type f -name '*_*.rs' | sort)

    for prefix in "${!prefix_counts[@]}"; do
      local count="${prefix_counts[$prefix]}"
      if ((count <= 1)); then
        continue
      fi

      while IFS= read -r hit; do
        [[ -z "$hit" ]] && continue
        local base
        base="$(basename "$hit" .rs)"
        local suffix="${base#${prefix}_}"
        echo "${hit}:1: repeated '${prefix}_*' file group in '${dir}'."
        echo "error: move '${base}.rs' under '${prefix}/${suffix}.rs' (or a '${prefix}' module) so names read as '${prefix}::...'."
        echo
      done <<< "${prefix_hits[$prefix]}"

      echo "note: this rule triggers only when 2+ sibling files share the '${prefix}_*' prefix."
      echo
      fail=1
    done

    unset prefix_counts
    unset prefix_hits
  done < <(find "${roots[@]}" -type d | sort)
}

fail=0
check_general_compound_prefix_groups

mapfile -t markers < <(collect_marked_modules)
for marker in "${markers[@]}"; do
  IFS='|' read -r marker_file module <<< "$marker"

  leaf_matches="$(check_module_leaf_imports "$module")"
  if [[ -n "$leaf_matches" ]]; then
    echo "$leaf_matches"
    echo
    echo "error: '${module}' is marked as a descriptive module namespace."
    echo "Use 'use ${module};' and qualify symbols at use sites (for example '${module##*::}::Window')."
    echo "Do not import leaf symbols via '${module}::...'."
    echo
    fail=1
  fi

  internal_matches="$(check_module_internal_surface_imports "$marker_file" "$module")"
  if [[ -n "$internal_matches" ]]; then
    echo "$internal_matches"
    echo
    echo "error: '${module}' is a descriptive module and should consume its own parent surface."
    echo "Inside that module tree, avoid sibling leaf imports like 'super::message::...'."
    echo "Prefer importing the descriptive parent module and qualifying from it (for example 'use crate::views::partials::chat;' then 'chat::Message')."
    echo
    fail=1
  fi

  check_family_prefix_breakdown "$marker_file" "$module"
  check_compound_sibling_modules "$marker_file" "$module"
done

if ((fail != 0)); then
  exit 1
fi

exit 0
