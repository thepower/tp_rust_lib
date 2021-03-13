//! This crate contains extern function declarations for
//! functions provided by wasm VM.

extern "C" {
    /// Appends string of specified length to VM's debug buffer.
    pub fn debug(len: usize, string: *const u8);

    /// Flushes VM's debug buffer to log file.
    pub fn flush();

    /// Tries to find storage entry by key of specified length and returns corresponding value's
    /// length.
    ///
    /// # Returns
    ///
    /// * value's length for existing entry;
    /// * 0 for non-existent entry.
    pub fn storage_value_size(key_size: usize, key: *const u8) -> usize;

    /// Tries to find storage entry by key of specified length and copy corresponding value to
    /// provided destination buffer.
    ///
    /// Use storage_value_size to obtain required destination buffer's length.
    pub fn storage_read(key_size: usize, key: *const u8, value_size: usize, dst: *mut u8);

    /// Creates new or replaces existing storage entry.
    pub fn storage_write(key_size: usize, key: *const u8, value_size: usize, src: *const u8);

    /// Removes all storage entries.
    pub fn storage_reset();

    /// Returns raw transaction length.
    ///
    /// Returns 0 if transaction is unavailable.
    pub fn get_tx_raw_size() -> usize;

    /// Copies raw transaction body to provided destination buffer.
    ///
    /// Use get_tx_raw_size to obtain required destination buffer's length.
    ///
    /// # Returns
    ///
    /// * True if full transaction body was successfully copied to destination buffer;
    /// * False if transaction is unavailable.
    pub fn get_tx_raw(dst: *mut u8) -> bool;

    /// Returns raw arguments length.
    pub fn get_args_raw_size() -> usize;

    /// Copies raw arguments to provided destination buffer.
    ///
    /// Use get_args_raw_size to obtain required destination buffer's length.
    ///
    /// # Returns
    ///
    /// * True if arguments were successfully copied to destination buffer;
    /// * False otherwise.
    pub fn get_args_raw(dst: *mut u8) -> bool;

    /// Returns raw balances length.
    pub fn get_balance_raw_size() -> usize;

    /// Copies raw balances to provided destination buffer.
    ///
    /// Use get_balances_raw_size to obtain required destination buffer's length.
    ///
    /// # Returns
    ///
    /// * True if balances were successfully copied to destination buffer;
    /// * False otherwise.
    pub fn get_balance_raw(dst: *mut u8) -> bool;


    /// Sets return value.
    ///
    /// ret should be valid msgpacked value.
    pub fn set_return(len: usize, ret: *const u8);

    /// Emits new transaction.
    ///
    /// tx should be valid msgpacked transaction body.
    pub fn emit_tx(len: usize, tx: *const u8);
}
