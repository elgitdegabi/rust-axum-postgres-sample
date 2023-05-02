use serde::Deserialize;

use crate::config::constants::{PAGINATION_DEFAULT_PAGE_NUMBER, PAGINATION_DEFAULT_PAGE_SIZE};

/// based on https://docs.rs/axum/latest/axum/extract/struct.Query.html
#[derive(Deserialize)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
}

// TODO use constants
impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: PAGINATION_DEFAULT_PAGE_NUMBER,
            per_page: PAGINATION_DEFAULT_PAGE_SIZE,
        }
    }
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes constant values validation
     * Expectation:
     * Constant values should be validated
     */
    #[test]
    fn when_constant_values_are_valid() {
        assert_eq!("127.0.0.1:6767", HOST);
        assert_eq!("server is running", SERVER_RUNNING_STATUS);
        assert_eq!("deleted ok", DELETE_OK_STATUS);
    }
}
