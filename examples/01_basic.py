#!/usr/bin/env python3
import pyskim
import time

opts = {
    'multi': True,
    'prompt': "> ",
    'header_lines': 1,
    'print_query': True,
    'bind': ['Enter:append-and-select+kill-line+unix-line-discard,Esc:accept'],
}


print(pyskim.quick_skim([
    "Select tags for https://github.com/lotabout/skim",
    "cli",
    "python",
    "github",
    "skim",
], **opts))

