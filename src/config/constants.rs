pub const HOST: &str = "0.0.0.0:6767";
pub const SERVER_RUNNING_STATUS: &str = "server is running";
pub const DELETE_OK_STATUS: &str = "deleted ok";

pub const PAGINATION_DEFAULT_PAGE_NUMBER: i64 = 0;
pub const PAGINATION_DEFAULT_PAGE_SIZE: i64 = 30;

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
