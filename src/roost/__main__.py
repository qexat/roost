#!/usr/bin/env python3
from argparse import ArgumentParser
from argparse import FileType
from collections.abc import Callable
from dataclasses import dataclass
from os import linesep
from sys import stderr
from sys import stdout
from typing import TextIO
from typing import TypeVar


DEFAULT_LINENO = 1
DEFAULT_PATH = "<stdin>"
DEFAULT_ERRNUM = 69

T = TypeVar("T")


@dataclass(slots=True)
class ErrorData:
	summary: str
	line: str
	message: str
	spos: int
	epos: int
	lineno: int = DEFAULT_LINENO
	path: str = DEFAULT_PATH
	errnum: int = DEFAULT_ERRNUM

	@property
	def errid(self) -> str:
		return "E" + str(self.errnum).zfill(4)

	def print(self, output: TextIO) -> None:
		lineno_len = len(str(self.lineno))
		empty_line = color(" " * (lineno_len + 1) + "| ", 4)

		string = bold(color(f"error[{self.errid}]", 1) + f": {self.summary}")
		string += linesep
		string += (
			" " * lineno_len + color("--> ", 4) + self.path + f":{self.lineno}:{self.spos}"
		)
		string += linesep
		string += empty_line
		string += linesep
		string += color(f"{self.lineno} | ", 4)
		string += (
			self.line[: self.spos]
			+ bold(color(self.line[self.spos : self.epos], 1))
			+ self.line[self.epos :]
		)
		string += linesep
		string += empty_line
		string += (
			" " * self.spos
			+ bold(color("^" * (self.epos - self.spos), 1))
			+ " "
			+ bold(color(self.message, 1))
		)
		string += linesep
		string += empty_line

		print(string, file=output)


def bold(string: str) -> str:
	return f"\x1b[1m{string}\x1b[0m"


def color(string: str, code: int) -> str:
	return f"\x1b[3{code}m{string}\x1b[39m"


def make_prompt(name: str, default: object | None) -> str:
	prompt = name

	if default is not None:
		prompt += color(f" ({default=})", 4)

	return bold(prompt + ": ")


def field(
	name: str, field_type: Callable[[str], T] = str, default: T | None = None,
) -> T:
	while True:
		result = input(make_prompt(name, default))
		if not result:
			if default is not None:
				return default
			print(bold(color(f"ERR: field {name!r} cannot be empty", 1)), file=stderr)
			continue
		try:
			return field_type(result)
		except (TypeError, ValueError):
			print(
				bold(
					color(
						f"ERR: {result!r} is not a valid " + field_type.__name__,
						3,
					),
				),
				file=stderr,
			)


def int_factory(
	name: str,
	min_value: int | None = None,
	max_value: int | None = None,
) -> Callable[[str], int]:
	def inner(raw_value: str) -> int:
		value = int(raw_value)

		if min_value is not None and value < min_value:
			raise ValueError
		if max_value is not None and value > max_value:
			raise ValueError

		return value

	inner.__name__ = name
	return inner


def print_line_helper(line: str) -> None:
	last_char_no_len = len(str(len(line))) + 1
	helper_len = last_char_no_len * len(line)

	print("─" * helper_len)

	for i in range(len(line)):
		print(str(i).center(last_char_no_len), end="")
	print()

	for c in line:
		print(str(c).center(last_char_no_len), end="")
	print()

	print("─" * helper_len)


def main() -> int:
	parser = ArgumentParser()
	parser.add_argument("--output", type=FileType("w"), default=stdout)
	args = parser.parse_args()

	try:
		summary = field("summary", str)
		line = field("line", str)

		print_line_helper(line)

		spos = field(
			"error start position",
			int_factory("spos", 0, len(line)),
			0,
		)
		epos = (
			field(
				"error end position",
				int_factory("epos", spos + 1, len(line) - 1),
				len(line) - 1,
			)
			+ 1
		)
		message = field("message", str)
		lineno = field("line number", int, DEFAULT_LINENO)
		path = field("path", str, DEFAULT_PATH)
		errnum = field("error number", int, DEFAULT_ERRNUM)

		print()

		err = ErrorData(summary, line, message, spos, epos, lineno, path, errnum)
		err.print(args.output)
	except KeyboardInterrupt:
		pass
	except EOFError:
		return 1

	return 0


if __name__ == "__main__":
	raise SystemExit(main())
