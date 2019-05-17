use crate::{cursor_condition::CursorCondition, filter_conversion::AliasedCondition};
use connector::QueryArguments;
use prisma_models::prelude::*;
use prisma_query::ast::{Aliasable, Comparable, ConditionTree, Joinable, Select};
use std::sync::Arc;

pub struct RelatedNodesBaseQuery<'a> {
    pub from_field: Arc<RelationField>,
    pub selected_fields: &'a SelectedFields,
    pub from_node_ids: &'a [GraphqlId],
    pub query: Select,
    pub order_by: Option<OrderBy>,
    pub is_reverse_order: bool,
    pub condition: ConditionTree,
    pub cursor: ConditionTree,
    pub window_limits: (u32, u32),
}

impl<'a> RelatedNodesBaseQuery<'a> {
    pub fn new(
        from_field: Arc<RelationField>,
        from_node_ids: &'a [GraphqlId],
        query_arguments: QueryArguments,
        selected_fields: &'a SelectedFields,
    ) -> RelatedNodesBaseQuery<'a> {
        let cursor = CursorCondition::build(&query_arguments, from_field.related_model());
        let window_limits = query_arguments.window_limits();

        let condition = query_arguments
            .filter
            .map(|f| f.aliased_cond(None))
            .unwrap_or(ConditionTree::NoCondition);

        let opposite_column = from_field.opposite_column().table(Relation::TABLE_ALIAS);
        let select = Select::from_table(from_field.related_model().table());

        let join = from_field
            .relation()
            .relation_table()
            .alias(Relation::TABLE_ALIAS)
            .on(from_field.related_model().id_column().equals(opposite_column));

        let query = selected_fields
            .columns()
            .into_iter()
            .fold(select, |acc, col| acc.column(col.clone()))
            .inner_join(join);

        let order_by = query_arguments.order_by;
        let is_reverse_order = query_arguments.last.is_some();

        Self {
            from_field,
            selected_fields,
            from_node_ids,
            query,
            order_by,
            is_reverse_order,
            condition,
            cursor,
            window_limits,
        }
    }
}
