use super::*;
use prisma_models::prelude::*;
use prisma_query::ast::*;

pub struct RelatedNodesWithUnionAll;

impl RelatedNodesQueryBuilder for RelatedNodesWithUnionAll {
    fn with_pagination<'a>(base: RelatedNodesBaseQuery<'a>) -> Query {
        let distinct_ids = {
            let mut ids = base.from_node_ids.to_vec();
            ids.dedup();

            ids
        };

        let base_query = base.query;
        let base_condition = base.condition.and(base.cursor);
        let from_field = base.from_field;

        let union = distinct_ids.into_iter().fold(UnionAll::default(), |acc, id| {
            let conditions = base_condition
                .clone()
                .and(from_field.relation_column().table(Relation::TABLE_ALIAS).equals(id));

            acc.add(base_query.clone().so_that(conditions))
        });

        Query::from(union)
    }
}
