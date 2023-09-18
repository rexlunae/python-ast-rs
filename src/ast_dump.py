import ast
#from io import StringIO

# Read the parse tree
def ast_dump(s, indent=None):
    print("Dump:", s)
    return ast.dump(s, indent=indent)
    print("...end")
