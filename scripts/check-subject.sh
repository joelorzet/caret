#!/usr/bin/env bash
# Holds one subject line to Conventional Commits.
#
# Used by the conventions workflow for both commit subjects and the title
# of a pull request, which is the subject once it is squashed.
set -uo pipefail

subject=${1:-}
what=${2:-subject}
kinds=${KINDS:-feat|fix|ref|hotfix|docs|ci|chore|test|perf|build|revert}
limit=72

fail() {
  echo "::error::$what: $1"
  echo "::error::  got: $subject"
  echo "::error::  want: <kind>(<scope>): <subject>, kind one of ${kinds//|/, }"
  exit 1
}

[ -n "$subject" ] || fail "is empty"
printf '%s' "$subject" | grep -Eq "^(${kinds})(\([a-z0-9._/-]+\))?!?: ." || fail "is not conventional"
printf '%s' "$subject" | grep -Eq ": [a-z]" || fail "starts with a capital after the kind"
printf '%s' "$subject" | grep -Eq "[.]$" && fail "ends with a full stop"
[ "${#subject}" -le "$limit" ] || fail "is ${#subject} characters, over the $limit limit"
exit 0
