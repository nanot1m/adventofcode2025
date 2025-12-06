#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <day-number>" >&2
  exit 1
fi

DAY_RAW="$1"
if ! [[ "$DAY_RAW" =~ ^[0-9]+$ ]]; then
  echo "Day must be a positive integer (got: $DAY_RAW)" >&2
  exit 1
fi
DAY=$((10#$DAY_RAW)) # strip leading zeros for file/URL

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$REPO_ROOT/.env"

if [[ -f "$ENV_FILE" ]]; then
  # shellcheck disable=SC1090
  set -a; source "$ENV_FILE"; set +a
fi

if [[ -z "${SESSION:-}" ]]; then
  echo 'Environment variable "SESSION" is not provided.' >&2
  echo 'Login at https://adventofcode.com/2025/auth/login and set the cookie value "session".' >&2
  exit 1
fi

YEAR=2025
OUT_DIR="$REPO_ROOT/input"
OUT_FILE="$OUT_DIR/day${DAY}.input.txt"
URL="https://adventofcode.com/${YEAR}/day/${DAY}/input"

mkdir -p "$OUT_DIR"

if [[ -f "$OUT_FILE" ]]; then
  echo "Input already exists at $OUT_FILE; delete it first to re-fetch."
  exit 0
fi

echo "Fetching input for day ${DAY} from Advent of Code ${YEAR}..."
curl -fLsS --cookie "session=${SESSION}" "$URL" -o "$OUT_FILE"
echo "Saved to $OUT_FILE"
