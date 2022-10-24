import ast
#from io import StringIO

# Read the parse tree 
def parse(string, filename):
    return ast.parse(string, filename=filename)

