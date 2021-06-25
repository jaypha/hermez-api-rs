#![macro_use]

// Internal use only

macro_rules! setter {
    ($name: ident, &'a [u32]) => {
        pub fn $name(&mut self, $name: &'a [u32]) -> &mut Self {
            self.$name = Some($name);
            self
        }
    };
    ($name: ident, &'a [&'a str]) => {
        pub fn $name(&mut self, $name: &'a [&'a str]) -> &mut Self {
            self.$name = Some($name);
            self
        }
    };
    ($name: ident, &'a str) => {
        pub fn $name(&mut self, $name: &'a str) -> &mut Self {
            self.$name = Some($name);
            self
        }
    };
    ($name: ident, $type: ident) => {
        pub fn $name(&mut self, $name: $type) -> &mut Self {
            self.$name = Some($name);
            self
        }
    };
}

macro_rules! setter_body {
    ($name: ident, &'a str) => {
        pub fn $name(&mut self, $name: &'a str) -> &mut Self {
            self.body.$name = Some($name);
            self
        }
    };
    ($name: ident, $type: ident) => {
        pub fn $name(&mut self, $name: $type) -> &mut Self {
            self.body.$name = Some($name);
            self
        }
    };
}

macro_rules! pagination_setters {
    () => {
        setter!(from_item, u64);
        setter!(order, PaginationOrder);
        setter!(limit, u64);
    };
}

macro_rules! fetch_stmt {
    ($self: ident, $pairs: ident, $name: ident, $param: expr, &str) => {
        if let Some($name) = &$self.$name {
            $pairs.append_pair($param, $name);
        }
    };
    ($self: ident, $pairs: ident, $name: ident, $param: expr, enum) => {
        if let Some($name) = &$self.$name {
            $pairs.append_pair(
                $param,
                serde_json::to_string(&$name)
                    .unwrap()
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap(),
            );
        }
    };
    ($self: ident, $pairs: ident, $name: ident, $param: expr) => {
        if let Some($name) = &$self.$name {
            $pairs.append_pair($param, &$name.to_string());
        }
    };
}

macro_rules! pagination_fetch_stmts {
    ($self: ident, $pairs: ident) => {
        fetch_stmt!($self, $pairs, from_item, "fromItem");
        fetch_stmt!($self, $pairs, order, "order");
        fetch_stmt!($self, $pairs, limit, "limit");
    };
}

macro_rules! is_more_than_one_defined {
    ( $( $x:expr ),+ ) => {
        (
            0 $(+ { if $x.is_some() { 1 } else {0} })*
        ) > 1
    }
}

macro_rules! test_required {
    ($self: ident, $x:ident) => {
        if $self.body.$x.is_none() {
            return Err(ErrorKind::Api(stringify!($x is a required parameter).to_owned()));
        }
    };
}

mod test {
    #[test]
    fn test() {
        let x: Option<bool> = None;

        assert!(!is_more_than_one_defined!(x, x, x));

        assert!(!is_more_than_one_defined!(Some(5u32), x));

        assert!(is_more_than_one_defined!(Some(5u32), Some(true)));

        assert!(is_more_than_one_defined!(Some(5u32), Some(true), x));
    }
}
