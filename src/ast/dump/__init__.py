import ast
#from io import StringIO

# Read the parse tree
def dump(s, indent=None, depth=0):
    if type(s) == list:
        l = []
        for i in s:
            l.append(dump(i, indent, depth=depth+1))
        return '[' + ', '.join(l) + ']'
    return ast.dump(s, indent=indent)
