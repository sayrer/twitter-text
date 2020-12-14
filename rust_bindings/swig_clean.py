
# Remove a few pieces of syntax from twitter-text.h.
# SWIG can't parse them, and doesn't need them.

import argparse

box_variadic = """
  template <typename... Fields>
  static Box in_place(Fields &&...);
"""

start_swig = "#ifndef SWIG\n"
end_swig = "#endif // SWIG\n"

def clean():
    parser = argparse.ArgumentParser()
    parser.add_argument("file")
    parser.add_argument("output")
    args = parser.parse_args()

    with open (args.output, "w") as output:
        with open (args.file, "r") as file:
            text = file.read()

            text = text.replace("#ifndef CXXBRIDGE1_PANIC", start_swig + "#ifndef CXXBRIDGE1_PANIC")
            text = text.replace("#endif // CXXBRIDGE1_PANIC", "#endif // CXXBRIDGE1_PANIC" + "\n" + end_swig)
            text = text.replace(box_variadic, start_swig + box_variadic + end_swig)

            output.write(text)                

if __name__ == "__main__":
    clean()
