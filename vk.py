#!/usr/bin/env python3

from __future__ import annotations

import argparse
import os
import re
import sys
from dataclasses import dataclass
from pathlib import Path


def get_dir_description(name: str) -> str:
   descriptions = {
      "LICENSES": "yaoi texts with ASCII art",
      "examples": "examples for bombs and instructions how to build them",
      "src": "source for our darknet marketplace",
      "target": "output of our marketplace's build system",
      "templates": "the HTML templates for the femboy auction system",
   }
   return descriptions.get(name, "")


def render_agents_template(template: str, dirs: list[tuple[str, str]]) -> str:
   pattern = re.compile(r"\{\{#each dirs \}\}.*?\{\{/each\}\}\n?", re.DOTALL)
   rendered_dirs = "".join(f"- {name}: {description}\n" for name, description in dirs)
   return pattern.sub(rendered_dirs, template)


@dataclass(frozen=True)
class ShowStatus:
   def explain(self) -> str:
      return "Showing the status."

   def execute(self) -> None:
      raise RuntimeError("This is not implemented yet. Sorry! 👉👈")


@dataclass(frozen=True)
class Symlink:
   from_path: Path
   to_path: Path

   def explain(self) -> str:
      return f"Linking from {self.from_path} to {self.to_path}."

   def execute(self) -> None:
      os.symlink(self.to_path, self.from_path)


@dataclass(frozen=True)
class WriteBasic:
   dirs: list[tuple[str, str]]
   evil: bool

   def explain(self) -> str:
      names = ", ".join(name for name, _ in self.dirs)
      mode = "enabled" if self.evil else "disabled"
      return f"Write the AGENTS.md with directories {names} and evil mode {mode}."

   def execute(self) -> None:
      template_name = "AGENTS_EVIL.md" if self.evil else "AGENTS.md"
      template_path = Path(__file__).resolve().parent / "src" / template_name
      template = template_path.read_text(encoding="utf-8")
      rendered = render_agents_template(template, self.dirs)
      Path("AGENTS.md").write_text(rendered, encoding="utf-8")


def build_parser() -> argparse.ArgumentParser:
   parser = argparse.ArgumentParser(prog="vk", description="Vibe Killer")
   parser.add_argument("-d", "--dry", action="store_true", default=False)
   subparsers = parser.add_subparsers(dest="command")

   init_parser = subparsers.add_parser("init", help="Init Vibe Killer in repository")
   init_parser.add_argument(
      "platforms",
      nargs="*",
      choices=["claude", "copilot", "cursor"],
   )
   init_parser.add_argument("--evil", action="store_true")

   subparsers.add_parser("status", help="Report on the status of Vibe Killer in the repository")
   return parser


def collect_dirs() -> list[tuple[str, str]]:
   dirs: list[tuple[str, str]] = []
   for entry in Path(".").iterdir():
      if entry.is_dir() and not entry.name.startswith("."):
         dirs.append((entry.name, get_dir_description(entry.name)))
   return dirs


def build_plan(args: argparse.Namespace) -> list[ShowStatus | Symlink | WriteBasic]:
   if args.command in (None, "status"):
      return [ShowStatus()]
   if args.command == "init":
      print(f"platforms = {args.platforms!r}", file=sys.stderr)
      return [
         WriteBasic(dirs=collect_dirs(), evil=args.evil),
         Symlink(from_path=Path("CLAUDE.md"), to_path=Path("AGENTS.md")),
      ]
   raise RuntimeError(f"Unknown command: {args.command}")


def main() -> int:
   parser = build_parser()
   args = parser.parse_args()
   plan = build_plan(args)

   if args.dry:
      print("Dry run.")

   try:
      for step in plan:
         if args.dry:
            print(step.explain())
         else:
            step.execute()
   except OSError as error:
      print(f"Could not operate on files: {error}", file=sys.stderr)
      return 1
   except RuntimeError as error:
      print(str(error), file=sys.stderr)
      return 1
   return 0


if __name__ == "__main__":
   raise SystemExit(main())
