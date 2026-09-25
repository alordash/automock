#![cfg(nightly)]
#![allow(stable_features)]
#![feature(rwlock_downgrade)]

use automock::*;

#[mock]
trait Trait {
    fn work(&self, v: i32);
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::*;
    use std::sync::*;

    // Used to run tests sequentially, otherwise `write_config` may cause deadlock
    // since `std::sync::RwLock` does not have priority policy for read and write locks.
    static TESTS_SEQ_SYNCER: LazyLock<Mutex<()>> = LazyLock::new(|| Default::default());

    fn seq_sync<'a>() -> MutexGuard<'a, ()> {
        TESTS_SEQ_SYNCER
            .lock()
            .expect("Unable to lock `TESTS_SYNCER`.")
    }

    #[test]
    fn UnexpectedValue_CallsCountLessThanLimit_PrintsAll() {
        let _lock = seq_sync();

        // Arrange
        let mut mock = TraitMock::new();
        let max_invalid_calls_listed_count = 4;
        let calls_count = max_invalid_calls_listed_count - 1;

        let mut write_config = write_config();
        write_config.max_invalid_calls_listed_count = max_invalid_calls_listed_count;
        let _read_settings_lock = RwLockWriteGuard::downgrade(write_config);

        let unexpected_v = 10;
        let expected_v = 20;

        // Act
        for _ in 0..calls_count {
            mock.work(unexpected_v);
        }
        let actual_error_msg = record_panic(|| mock.received().work(expected_v, Times::Once));

        // Assert
        let calls_error_msgs = format!(
            "
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}"
        )
        .repeat(calls_count);
        let expected_error_msg = format!(
                "Expected to receive a call exactly once matching:
	Trait::work((i32): equal to {expected_v})
Actually received no matching calls
Received {calls_count} non-matching calls (non-matching arguments indicated with '*' characters):{calls_error_msgs}"
            );
        assert_eq!(Some(expected_error_msg), actual_error_msg);
    }

    #[test]
    fn UnexpectedValue_CallsCountMoreThanLimit_PrintsTrimmed() {
        let _lock = seq_sync();

        // Arrange
        let mut mock = TraitMock::new();
        let max_invalid_calls_listed_count = 4;
        let calls_count = max_invalid_calls_listed_count + 1;

        let mut write_config = write_config();
        write_config.max_invalid_calls_listed_count = max_invalid_calls_listed_count;
        let _read_settings_lock = RwLockWriteGuard::downgrade(write_config);

        let unexpected_v = 10;
        let expected_v = 20;

        // Act
        for _ in 0..calls_count {
            mock.work(unexpected_v);
        }
        let actual_error_msg = record_panic(|| mock.received().work(expected_v, Times::Once));

        // Assert
        let calls_error_msgs = format!(
            "
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}"
        )
        .repeat(max_invalid_calls_listed_count);
        let expected_error_msg = format!(
                "Expected to receive a call exactly once matching:
	Trait::work((i32): equal to {expected_v})
Actually received no matching calls
Received {calls_count} non-matching calls (listing only first {max_invalid_calls_listed_count}) (non-matching arguments indicated with '*' characters):{calls_error_msgs}"
            );
        assert_eq!(Some(expected_error_msg), actual_error_msg);
    }

    #[test]
    fn ExpectedValue_CallsCountLessThanLimit_PrintsAll() {
        let _lock = seq_sync();

        // Arrange
        let mut mock = TraitMock::new();
        let max_invalid_calls_listed_count = 4;
        let calls_count = max_invalid_calls_listed_count - 1;

        let mut write_config = write_config();
        write_config.max_invalid_calls_listed_count = max_invalid_calls_listed_count;
        let _read_settings_lock = RwLockWriteGuard::downgrade(write_config);

        let expected_v = 10;

        // Act
        for _ in 0..calls_count {
            mock.work(expected_v);
        }
        let actual_error_msg = record_panic(|| mock.received().work(expected_v, Times::Once));

        // Assert
        let calls_error_msgs = format!(
            "
	work({expected_v})"
        )
        .repeat(calls_count);
        let expected_error_msg = format!(
            "Expected to receive a call exactly once matching:
	Trait::work((i32): equal to {expected_v})
Actually received {calls_count} matching calls:{calls_error_msgs}
Received no non-matching calls"
        );
        assert_eq!(Some(expected_error_msg), actual_error_msg);
    }

    #[test]
    fn ExpectedValue_CallsCountMoreThanLimit_PrintsTrimmed() {
        let _lock = seq_sync();

        // Arrange
        let mut mock = TraitMock::new();
        let max_invalid_calls_listed_count = 4;
        let calls_count = max_invalid_calls_listed_count + 1;

        let mut write_config = write_config();
        write_config.max_invalid_calls_listed_count = max_invalid_calls_listed_count;
        let _read_settings_lock = RwLockWriteGuard::downgrade(write_config);

        let expected_v = 10;

        // Act
        for _ in 0..calls_count {
            mock.work(expected_v);
        }
        let actual_error_msg = record_panic(|| mock.received().work(expected_v, Times::Once));

        // Assert
        let calls_error_msgs = format!(
            "
	work({expected_v})"
        )
        .repeat(max_invalid_calls_listed_count);
        let expected_error_msg = format!(
            "Expected to receive a call exactly once matching:
	Trait::work((i32): equal to {expected_v})
Actually received {calls_count} matching calls (listing only first {max_invalid_calls_listed_count}):{calls_error_msgs}
Received no non-matching calls"
        );
        assert_eq!(Some(expected_error_msg), actual_error_msg);
    }

    #[test]
    fn NoOtherCalls_CallsCountLessThanLimit_PrintsAll() {
        let _lock = seq_sync();

        // Arrange
        let mut mock = TraitMock::new();
        let max_invalid_calls_listed_count = 4;
        let calls_count = max_invalid_calls_listed_count - 1;

        let mut write_config = write_config();
        write_config.max_invalid_calls_listed_count = max_invalid_calls_listed_count;
        let _read_settings_lock = RwLockWriteGuard::downgrade(write_config);

        let expected_v = 10;

        // Act
        for _ in 0..calls_count {
            mock.work(expected_v);
        }
        let actual_error_msg = record_panic(|| mock.received().no_other_calls());

        // Assert
        let calls_error_msgs: String = (1..=calls_count)
            .map(|i| {
                format!(
                    "
{i}. Trait::work({expected_v})"
                )
            })
            .collect();
        let expected_error_msg = format!(
            "Did not expect to receive any other calls. Received {calls_count} unexpected calls:{calls_error_msgs}"
        );
        assert_eq!(Some(expected_error_msg), actual_error_msg);
    }

    #[test]
    fn NoOtherCalls_CallsCountMoreThanLimit_PrintsTrimmed() {
        let _lock = seq_sync();

        // Arrange
        let mut mock = TraitMock::new();
        let max_invalid_calls_listed_count = 4;
        let calls_count = max_invalid_calls_listed_count + 1;

        let mut write_config = write_config();
        write_config.max_invalid_calls_listed_count = max_invalid_calls_listed_count;
        let _read_settings_lock = RwLockWriteGuard::downgrade(write_config);

        let expected_v = 10;

        // Act
        for _ in 0..calls_count {
            mock.work(expected_v);
        }
        let actual_error_msg = record_panic(|| mock.received().no_other_calls());

        // Assert
        let calls_error_msgs: String = (1..=max_invalid_calls_listed_count)
            .map(|i| {
                format!(
                    "
{i}. Trait::work({expected_v})"
                )
            })
            .collect();
        let expected_error_msg = format!(
            "Did not expect to receive any other calls. Received {calls_count} unexpected calls (listing only first {max_invalid_calls_listed_count}):{calls_error_msgs}"
        );
        assert_eq!(Some(expected_error_msg), actual_error_msg);
    }
}
