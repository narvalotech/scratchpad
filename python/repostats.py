#!/usr/bin/env python3

import subprocess
import sys
import os
from collections import defaultdict

def get_git_stats(repo_path, months=None):
    repo_path = os.path.abspath(repo_path)
    if not os.path.isdir(os.path.join(repo_path, ".git")):
        print(f"Error: '{repo_path}' is not a valid Git repository.")
        return None

    # Base command
    cmd = ['git', 'log', '--pretty=format:HOUR:%ad', '--date=format:%H', '--numstat']
    
    # Add time filter if requested
    if months:
        cmd.append(f'--since={months}.months')
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True, cwd=repo_path)
    except subprocess.CalledProcessError as e:
        print(f"Error accessing Git logs: {e}")
        return None

    commits_per_hour = defaultdict(int)
    diff_per_hour = defaultdict(int)
    current_hour = None

    for line in result.stdout.splitlines():
        line = line.strip()
        if line.startswith('HOUR:'):
            try:
                current_hour = int(line.split(':')[1])
                commits_per_hour[current_hour] += 1
            except (IndexError, ValueError):
                continue
        elif line and current_hour is not None:
            parts = line.split()
            if len(parts) >= 3 and parts[0].isdigit() and parts[1].isdigit():
                diff_per_hour[current_hour] += (int(parts[0]) + int(parts[1]))
    
    return commits_per_hour, diff_per_hour

def draw_ascii_bar(data, title, label, char):
    print(f"\n{title.center(70)}")
    print("-" * 70)
    
    vals = data.values()
    max_val = max(vals) if vals else 0
    
    if max_val == 0:
        print("No data found for this period.")
        return

    max_bar_width = 40
    scale = max_bar_width / max_val if max_val > max_bar_width else 1

    for hour in range(24):
        val = data.get(hour, 0)
        bar_len = int(val * scale)
        bar = char * bar_len
        print(f"{hour:02d}:00 | {bar:<40} | {val} {label}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python repostats.py /path/to/repo [months_back]")
        sys.exit(1)

    path_arg = sys.argv[1]
    months_arg = sys.argv[2] if len(sys.argv) > 2 else None
    
    stats = get_git_stats(path_arg, months_arg)
    
    if stats:
        commits, diffs = stats
        period_label = f" (Last {months_arg} months)" if months_arg else " (All time)"
        
        draw_ascii_bar(commits, f"COMMITS PER HOUR{period_label}", "commits", "█")
        print("\n" + "="*70)
        draw_ascii_bar(diffs, f"DIFF SIZE PER HOUR{period_label}", "lines", "░")
