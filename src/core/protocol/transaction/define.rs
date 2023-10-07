
// list
define_transactions_type_and_create_func!{

    0u8  TRANSACTION_TYPE_0_COINBASE:   TransactionCoinbase
    1u8  TRANSACTION_TYPE_1_DEPRECATED: TransactionType1Deprecated
    2u8  TRANSACTION_TYPE_2:            TransactionType2

}

// type1
create_common_transaction_struct!{TRANSACTION_TYPE_1_DEPRECATED, TransactionType1Deprecated}

// type2
create_common_transaction_struct!{TRANSACTION_TYPE_2, TransactionType2}

