from hypothesis import strategies as _st

from ._expressions import expression_strategy
from ._factories import MAX_STATEMENTS_SIZE, to_statement_strategy

statement_strategy = to_statement_strategy(expression_strategy)
statement_list_strategy = _st.lists(
    statement_strategy, max_size=MAX_STATEMENTS_SIZE
)
