#[macro_export]
macro_rules! day_branch {
    ($mod:ident, $step:ident, $data:ident, $expected:ident) => {{
        if let Some(expected_value) = $expected {
            days::$mod::run_test($step, $data, expected_value)?;
        } else {
            days::$mod::run($step, $data)?;
        }
        Ok(())
    }};
}
