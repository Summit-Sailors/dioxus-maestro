use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityType {
	/// Order fills (both partial and full fills).
	///
	/// This variant will only ever be set for trade activities.
	#[serde(rename = "FILL")]
	Fill,
	/// Cash transactions (both CSD and CSW).
	#[serde(rename = "TRANS")]
	Transaction,
	///  Miscellaneous or rarely used activity types (All types except those in TRANS, DIV, or FILL).
	#[serde(rename = "MISC")]
	Miscellaneous,
	/// ACATS IN/OUT (Cash).
	#[serde(rename = "ACATC")]
	AcatsInOutCash,
	/// ACATS IN/OUT (Securities).
	#[serde(rename = "ACATS")]
	AcatsInOutSecurities,
	/// Cash deposit(+).
	#[serde(rename = "CSD")]
	CashDeposit,
	/// Cash withdrawal(-).
	#[serde(rename = "CSW")]
	CashWithdrawal,
	/// Dividends.
	#[serde(rename = "DIV")]
	Dividend,
	/// Dividend (capital gain long term).
	#[serde(rename = "DIVCGL")]
	CapitalGainLongTerm,
	/// Dividend (capital gain short term).
	#[serde(rename = "DIVCGS")]
	CapitalGainShortTerm,
	/// Dividend fee.
	#[serde(rename = "DIVFEE")]
	DividendFee,
	/// Dividend adjusted (Foreign Tax Withheld).
	#[serde(rename = "DIVFT")]
	DividendAdjusted,
	/// Dividend adjusted (NRA Withheld).
	#[serde(rename = "DIVNRA")]
	DividendAdjustedNraWithheld,
	/// Dividend return of capital.
	#[serde(rename = "DIVROC")]
	DividendReturnOfCapital,
	/// Dividend adjusted (Tefra Withheld).
	#[serde(rename = "DIVTW")]
	DividendAdjustedTefraWithheld,
	/// Dividend (tax exempt).
	#[serde(rename = "DIVTXEX")]
	DividendTaxExtempt,
	/// Interest (credit/margin).
	#[serde(rename = "INT")]
	Interest,
	/// Interest adjusted (NRA Withheld).
	#[serde(rename = "INTNRA")]
	InterestAdjustedNraWithheld,
	/// Interest adjusted (Tefra Withheld).
	#[serde(rename = "INTTW")]
	InterestAdjustedTefraWithheld,
	/// Journal entry.
	#[serde(rename = "JNL")]
	JournalEntry,
	/// Journal entry (cash).
	#[serde(rename = "JNLC")]
	JournalEntryCash,
	/// Journal entry (stock).
	#[serde(rename = "JNLS")]
	JournalEntryStock,
	/// Merger/Acquisition.
	#[serde(rename = "MA")]
	Acquisition,
	/// Name change.
	#[serde(rename = "NC")]
	NameChange,
	/// Option assignment.
	#[serde(rename = "OPASN")]
	OptionAssignment,
	/// Option expiration.
	#[serde(rename = "OPEXP")]
	OptionExpiration,
	/// Option exercise.
	#[serde(rename = "OPXRC")]
	OptionExercise,
	/// Pass Thru Charge.
	#[serde(rename = "PTC")]
	PassThruCharge,
	/// Pass Thru Rebate.
	#[serde(rename = "PTR")]
	PassThruRebate,
	/// SEC and FINRA fees.
	#[serde(rename = "FEE")]
	Fee,
	/// Reorg CA.
	#[serde(rename = "REORG")]
	Reorg,
	/// Symbol change.
	#[serde(rename = "SC")]
	SymbolChange,
	/// Stock spinoff.
	#[serde(rename = "SPIN")]
	StockSpinoff,
	/// Stock split.
	#[serde(rename = "SPLIT")]
	StockSplit,
}
