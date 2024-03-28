common_grammar = """
statement: expression_statement
expression_statement   : expression ";"

expression             : annotation_expression
                       | assignment
                       | expression binary_operator term
                       | expression comparison_operator term
annotation_expression  : term
                       | member_access
                       | call
assignment             : assignment_target "=" expression
binary_operator        : "+" | "-" | "*" | "/"
comparison_operator    : "<" | ">" | "<=" | ">=" | "==" | "!="
integer_literal        : /(?a)-?\\d+_(I|U)(8|16|32|64|Size)/
floating_point_literal : \
    /(?a)-?(\\d+(\\.\\d*)?|\\.\\d+)((e|E)(\\+|-)?\\d+)?_F(32|64)/
call                   : term "(" [expression ("," expression)* ","?] ")"
member_access          : annotation_expression "." identifier
tuple                  : singleton_tuple
                       | non_singleton_tuple
singleton_tuple        : "(" expression "," ")"
non_singleton_tuple    : "(" [expression ("," expression)+ ","?] ")"
term                   : floating_point_literal
                       | integer_literal
                       | identifier
                       | conditional
                       | function_definition
                       | unary_operator term
                       | tuple
                       | "(" expression ")"
                       | block
block                  : "{" [statement]* "}"
function_definition    : \
    "Function" "(" [annotated_identifier ("," annotated_identifier)* ","?] ")"\
    "->" term block
annotated_identifier   : identifier ":" annotation_expression
conditional            : \
    "if" WS expression block ["else" WS (conditional | block)]
unary_operator         : "-"
identifier             : /(?a)\\b(?!(Function|if|else)\\b)[A-Z_a-z]\\w*/

assignment_target              : assignment_target_expression
assignment_target_expression   : assignment_target_term
                               | member_access
                               | assignment_target_tuple
assignment_target_tuple        : assignment_target_singleton_tuple
                               | assignment_target_non_singleton_tuple
assignment_target_singleton_tuple     : \
    "(" assignment_target_expression "," ")"
assignment_target_non_singleton_tuple : \
    "(" \
    [assignment_target_expression ("," assignment_target_expression)+ ","?] \
    ")"
assignment_target_term                : identifier
                                      | annotated_identifier
                                      | "(" assignment_target_expression ")"

%import common.C_COMMENT
%import common.CNAME
%import common.CPP_COMMENT
%import common.NEWLINE
%import common.WS
%ignore WS
%ignore WS (C_COMMENT | CPP_COMMENT NEWLINE)
"""
