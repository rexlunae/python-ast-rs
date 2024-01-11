import ast

# Read the parse tree
def parse(string, filename):
    return ast.parse(string, filename=filename)
