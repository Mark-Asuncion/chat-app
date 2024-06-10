use super::{QueryValue, ORDER, Operation, filter::{Filter, JoinFilter, VecFilterDisplay}, VecQueryValue};

type UpdateValue = (String, QueryValue);
type OrderValue  = ( String, ORDER );
pub struct QueryBuilder {
    operation:      Option<Operation>,
    filters:        Option< Vec<Filter> >,
    update_values:  Option< Vec<UpdateValue> >,
    join_filters:   Option< Vec<JoinFilter> >,
    insert:         Option< Vec<Vec<QueryValue>> >,
    order_by:       Option<OrderValue>
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            operation:      None,
            filters:        None,
            join_filters:   None,
            insert:         None,
            order_by:       None,
            update_values:  None
        }
    }

    fn _clear(&mut self) {
            self.operation =      None;
            self.filters =        None;
            self.join_filters =   None;
            self.insert =         None;
            self.order_by =       None;
            self.update_values =  None;
    }

    pub fn build(&mut self) -> String {
        if let None = self.operation {
            return String::new();
        }

        match self.operation.as_ref().unwrap() {
            Operation::Select(table, columns) => {
                let mut query = format!("SELECT ");
                if let Some(v) = columns {
                    let mut cols = String::new();
                    for (i, col) in v.iter().enumerate() {
                        cols += &col;
                        if i+1 != v.len() {
                            cols += ",";
                        }
                    }
                    query += &format!("{} FROM {}\n", cols, table);
                }
                else {
                    query += &format!("* FROM {}\n", table);
                }
                if let Some(filters) = self.filters.as_ref() {
                    let ft: String = filters.to_string(&self.join_filters);
                    query += &format!("WHERE {}\n", ft);
                }
                if let Some((by, order)) = self.order_by.as_ref() {
                    query += &format!("ORDER BY {} {}", by, order);
                }
                self._clear();
                query += ";";
                query
            },
            Operation::Update(table) => {
                let mut query = format!("UPDATE {}\n", table);
                if let Some(sets) = self.update_values.as_ref() {
                    for (i, set) in sets.iter().enumerate() {
                        query += &format!("{} = {}", set.0, set.1);
                        if i+1 != sets.len() {
                            query += ", ";
                        }
                    }
                    query += "\n";
                }
                if let Some(filters) = self.filters.as_ref() {
                    let ft: String = filters.to_string(&self.join_filters);
                    query += &format!("WHERE {}\n", ft);
                }
                self._clear();
                query += ";";
                query
            }
            Operation::Insert(table, columns) => {
                let mut cols = String::new();
                for (i, col) in columns.iter().enumerate() {
                    cols += &col;
                    if i+1 != columns.len() {
                        cols += ",";
                    }
                }
                let mut query = format!("INSERT INTO {} ({})\nVALUES ", table, cols);
                if let Some(values) = self.insert.as_ref() {
                    for (i, v) in values.iter().enumerate() {
                        query += &v.vqv_get();
                        if i+1 != values.len() {
                            query += ",";
                        }
                    }
                }
                else {
                    return String::new();
                }
                self._clear();
                query += ";";
                query
            }
        }
    }

    pub fn select(&mut self, table: &str, columns: Option<Vec<&str>>) -> &mut Self {
        let mut cols = None;
        if let Some(v) = columns {
            if !v.is_empty() {
                cols = Some(v.iter().map(|s| s.to_string()).collect());
            }
        }

        self.operation = Some(Operation::Select(table.into(), cols));
        self
    }

    pub fn update(&mut self, table: &str) -> &mut Self {
        self.operation = Some(Operation::Update(table.into()));
        self
    }

    pub fn insert(&mut self, table: &str, columns: Vec<&str>) -> &mut Self {
        let cols = columns.iter().map(|s| s.to_string()).collect();
        self.operation = Some(Operation::Insert(table.into(), cols));
        self
    }

    fn _get_mut_filters(&mut self) -> &mut Vec<Filter> {
        if let None = self.filters {
            self.filters = Some(vec![]);
        }
        self.filters.as_mut().unwrap()
    }

    fn _get_mut_join_filters(&mut self) -> &mut Vec<JoinFilter> {
        if let None = self.join_filters {
            self.join_filters = Some(vec![]);
        }
        self.join_filters.as_mut().unwrap()
    }

    pub fn filter(&mut self, filter: Filter) -> &mut Self {
        let filters = self._get_mut_filters();
        filters.push(filter);
        self
    }

    pub fn and(&mut self) -> &mut Self {
        let j = self._get_mut_join_filters();
        j.push(JoinFilter::And);
        self
    }

    pub fn or(&mut self) -> &mut Self {
        let j = self._get_mut_join_filters();
        j.push(JoinFilter::Or);
        self
    }

    pub fn value(&mut self, value: Vec<QueryValue>) -> &mut Self {
        let v: &mut Vec<Vec<QueryValue>>;
        if let None = self.insert {
            self.insert = Some(vec![]);
        }
        v = self.insert.as_mut().unwrap();
        v.push(value);
        self
    }

    pub fn set(&mut self, update_value: UpdateValue) -> &mut Self {
        let v: &mut Vec<UpdateValue>;
        if let None = self.update_values {
            self.update_values = Some(vec![]);
        }
        v = self.update_values.as_mut().unwrap();
        v.push(update_value);
        self
    }

    pub fn order_by(&mut self, name: &str, order: ORDER) -> &mut Self {
        self.order_by = Some((name.into(), order));
        self
    }
}
