use prisma_models::PrismaValue;
use query_core::{Operation, QueryValue, Selection};

use crate::{
    merge_fields, Include, IncludeType, ModelOperation, ModelQuery, ModelReadOperation, ModelTypes,
    OrderByQuery, PaginatedQuery, PrismaClientInternals, Query, QueryConvert, Select, SelectType,
    WhereInput, WhereQuery, WithQuery,
};

use super::SerializedWhereInput;

pub struct FindMany<'a, Actions: ModelTypes> {
    client: &'a PrismaClientInternals,
    pub where_params: Vec<Actions::Where>,
    pub with_params: Vec<Actions::With>,
    pub order_by_params: Vec<Actions::OrderBy>,
    pub cursor_params: Vec<Actions::Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<'a, Actions: ModelTypes> FindMany<'a, Actions> {
    pub fn new(client: &'a PrismaClientInternals, where_params: Vec<Actions::Where>) -> Self {
        Self {
            client,
            where_params,
            with_params: vec![],
            order_by_params: vec![],
            cursor_params: vec![],
            skip: None,
            take: None,
        }
    }

    pub fn with(mut self, param: impl Into<Actions::With>) -> Self {
        self.with_params.push(param.into());
        self
    }

    pub fn order_by(mut self, param: Actions::OrderBy) -> Self {
        self.order_by_params.push(param);
        self
    }

    pub fn cursor(mut self, param: Actions::Cursor) -> Self {
        self.cursor_params.push(param);
        self
    }

    pub fn skip(mut self, skip: i64) -> Self {
        self.skip = Some(skip);
        self
    }

    pub fn take(mut self, take: i64) -> Self {
        self.take = Some(take);
        self
    }

    fn to_selection(
        where_params: Vec<Actions::Where>,
        order_by_params: Vec<Actions::OrderBy>,
        cursor_params: Vec<Actions::Cursor>,
        skip: Option<i64>,
        take: Option<i64>,
        nested_selections: impl IntoIterator<Item = Selection>,
    ) -> Selection {
        Self::base_selection(
            [
                (!where_params.is_empty()).then(|| {
                    (
                        "where".to_string(),
                        PrismaValue::Object(merge_fields(
                            where_params
                                .into_iter()
                                .map(WhereInput::serialize)
                                .map(|s| (s.field, s.value.into()))
                                .collect(),
                        ))
                        .into(),
                    )
                }),
                (!order_by_params.is_empty()).then(|| {
                    (
                        "orderBy".to_string(),
                        PrismaValue::List(
                            order_by_params
                                .into_iter()
                                .map(Into::into)
                                .map(|v| PrismaValue::Object(vec![v]))
                                .collect(),
                        )
                        .into(),
                    )
                }),
                (!cursor_params.is_empty()).then(|| {
                    (
                        "cursor".to_string(),
                        PrismaValue::Object(
                            cursor_params
                                .into_iter()
                                .map(Into::into)
                                .map(WhereInput::serialize)
                                .map(SerializedWhereInput::transform_equals)
                                .collect(),
                        )
                        .into(),
                    )
                }),
                skip.map(|skip| ("skip".to_string(), QueryValue::Int(skip as i64))),
                take.map(|take| ("take".to_string(), QueryValue::Int(take as i64))),
            ]
            .into_iter()
            .flatten(),
            nested_selections,
        )
    }

    pub fn select<S: SelectType<ModelData = Actions::Data>>(
        self,
        select: S,
    ) -> Select<'a, Vec<S::Data>> {
        Select::new(
            self.client,
            Operation::Read(Self::to_selection(
                self.where_params,
                self.order_by_params,
                self.cursor_params,
                self.skip,
                self.take,
                select.to_selections(),
            )),
        )
    }

    pub fn include<I: IncludeType<ModelData = Actions::Data>>(
        self,
        include: I,
    ) -> Include<'a, Vec<I::Data>> {
        Include::new(
            self.client,
            Operation::Read(Self::to_selection(
                self.where_params,
                self.order_by_params,
                self.cursor_params,
                self.skip,
                self.take,
                include.to_selections(),
            )),
        )
    }

    pub async fn exec(self) -> super::Result<Vec<Actions::Data>> {
        super::exec(self).await
    }
}

impl<'a, Actions: ModelTypes> QueryConvert for FindMany<'a, Actions> {
    type RawType = Vec<Actions::Data>;
    type ReturnValue = Self::RawType;

    fn convert(raw: Self::RawType) -> Self::ReturnValue {
        raw
    }
}

impl<'a, Actions: ModelTypes> Query<'a> for FindMany<'a, Actions> {
    fn graphql(self) -> (Operation, &'a PrismaClientInternals) {
        let mut scalar_selections = Actions::scalar_selections();

        scalar_selections.extend(self.with_params.into_iter().map(Into::into));

        (
            Operation::Read(Self::to_selection(
                self.where_params,
                self.order_by_params,
                self.cursor_params,
                self.skip,
                self.take,
                scalar_selections,
            )),
            self.client,
        )
    }
}

impl<'a, Actions: ModelTypes> ModelQuery<'a> for FindMany<'a, Actions> {
    type Types = Actions;

    const TYPE: ModelOperation = ModelOperation::Read(ModelReadOperation::FindMany);
}

impl<'a, Actions: ModelTypes> WhereQuery<'a> for FindMany<'a, Actions> {
    fn add_where(&mut self, param: Actions::Where) {
        self.where_params.push(param);
    }
}

impl<'a, Actions: ModelTypes> WithQuery<'a> for FindMany<'a, Actions> {
    fn add_with(&mut self, param: impl Into<Actions::With>) {
        self.with_params.push(param.into());
    }
}

impl<'a, Actions: ModelTypes> OrderByQuery<'a> for FindMany<'a, Actions> {
    fn add_order_by(&mut self, param: Actions::OrderBy) {
        self.order_by_params.push(param);
    }
}

impl<'a, Actions: ModelTypes> PaginatedQuery<'a> for FindMany<'a, Actions> {
    fn add_cursor(&mut self, param: Actions::Cursor) {
        self.cursor_params.push(param);
    }

    fn set_skip(&mut self, skip: i64) {
        self.skip = Some(skip);
    }

    fn set_take(&mut self, take: i64) {
        self.take = Some(take);
    }
}

#[derive(Clone)]
pub struct ManyArgs<Actions: ModelTypes> {
    pub where_params: Vec<Actions::Where>,
    pub with_params: Vec<Actions::With>,
    pub order_by_params: Vec<Actions::OrderBy>,
    pub cursor_params: Vec<Actions::Cursor>,
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

impl<Actions: ModelTypes> ManyArgs<Actions> {
    pub fn new(where_params: Vec<Actions::Where>) -> Self {
        Self {
            where_params,
            with_params: vec![],
            order_by_params: vec![],
            cursor_params: vec![],
            skip: None,
            take: None,
        }
    }

    pub fn with(mut self, param: impl Into<Actions::With>) -> Self {
        self.with_params.push(param.into());
        self
    }

    pub fn order_by(mut self, param: Actions::OrderBy) -> Self {
        self.order_by_params.push(param);
        self
    }

    pub fn cursor(mut self, param: Actions::Cursor) -> Self {
        self.cursor_params.push(param);
        self
    }

    pub fn skip(mut self, skip: i64) -> Self {
        self.skip = Some(skip);
        self
    }

    pub fn take(mut self, take: i64) -> Self {
        self.take = Some(take);
        self
    }

    pub fn to_graphql(self) -> (Vec<(String, QueryValue)>, Vec<Selection>) {
        let (mut arguments, mut nested_selections) = (vec![], vec![]);

        if self.with_params.len() > 0 {
            nested_selections = self.with_params.into_iter().map(Into::into).collect()
        }

        if self.where_params.len() > 0 {
            arguments.push((
                "where".to_string(),
                PrismaValue::Object(
                    self.where_params
                        .into_iter()
                        .map(WhereInput::serialize)
                        .map(Into::into)
                        .collect(),
                )
                .into(),
            ));
        }

        if self.order_by_params.len() > 0 {
            arguments.push((
                "orderBy".to_string(),
                PrismaValue::List(
                    self.order_by_params
                        .into_iter()
                        .map(Into::into)
                        .map(|v| PrismaValue::Object(vec![v]))
                        .collect(),
                )
                .into(),
            ));
        }

        if self.cursor_params.len() > 0 {
            arguments.push((
                "cursor".to_string(),
                PrismaValue::Object(
                    self.cursor_params
                        .into_iter()
                        .map(Into::into)
                        .map(WhereInput::serialize)
                        .map(SerializedWhereInput::transform_equals)
                        .collect(),
                )
                .into(),
            ));
        }

        self.skip
            .map(|skip| arguments.push(("skip".to_string(), QueryValue::Int(skip))));
        self.take
            .map(|take| arguments.push(("take".to_string(), QueryValue::Int(take))));

        (arguments, nested_selections)
    }
}
