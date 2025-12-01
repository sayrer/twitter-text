# Copyright 2019 Robert Sayre
# Licensed under the Apache License, Version 2.0
# http://www.apache.org/licenses/LICENSE-2.0

import argparse
import sys

import patricia
import yaml


def main():
    parser = argparse.ArgumentParser(description="Generate TLD trie for Pest parser")
    parser.add_argument(
        "--input", required=True, help="Input YAML file containing TLDs"
    )
    parser.add_argument(
        "--output", required=True, help="Output file for generated Pest grammar"
    )
    args = parser.parse_args()

    with open(args.input, "r", encoding="utf-8") as stream:
        try:
            tld_structure = yaml.safe_load(stream)
        except yaml.YAMLError as exc:
            print(exc, file=sys.stderr)
            sys.exit(1)

        tlds = set()
        for kind in tld_structure.values():
            for element in kind:
                idna = element.encode("idna").decode("ascii")
                if idna != element:
                    tlds.add(idna)
                tlds.add(element)

    t = patricia.trie()
    for word in tlds:
        t[word] = True

    def values(node, edges):
        if node._value is not patricia.__NON_TERMINAL__:
            yield edges, node._value
        for edge, child in node._edges.values():
            for edge, value in values(child, edges + [edge]):
                yield edge, value

    root = dict()
    terminal = "_terminal_"
    for edges, value in values(t, []):
        current_dict = root
        for edge in edges:
            current_dict = current_dict.setdefault(edge, {})
        current_dict[terminal] = True

    def gen_prelude(prefix, indent, has_children):
        prelude = ""
        if prefix != "":
            prelude = indent + prefix
            if has_children:
                prelude += " ~" + "\n" + indent + "  ("
        return prelude

    def is_ascii(s):
        return all(ord(c) < 128 for c in s)

    def gen_literal(s):
        literal = ""
        if is_ascii(s):
            literal += "^"
        return literal + '"%s"' % s

    def print_pest(prefix, node, indent, output_file):
        is_terminal = node.pop(terminal, False)
        has_children = len(node.keys()) > 0
        prelude = gen_prelude(prefix, indent, has_children)
        if prelude != "":
            print(prelude, file=output_file)

        # Make sure ascii is at the top of the list,
        # because Pest has an ordered-choice operator.
        keys = sorted(node.keys())

        for idx, k in enumerate(keys):
            literal = gen_literal(k)
            if idx > 0:
                literal = "| " + literal
            print_pest(literal, node[k], indent + "   ", output_file)

        if is_terminal and has_children:
            print(indent + "  )?", file=output_file)
        elif prelude != "" and has_children:
            print(indent + "  )", file=output_file)

    with open(args.output, "w", encoding="utf-8") as output_file:
        print("tld = _{", file=output_file)
        print_pest("", root, "  ", output_file)
        print("}", file=output_file)


if __name__ == "__main__":
    main()
