/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_registers {
    NFT_REG_VERDICT = 0,
    NFT_REG_1 = 1,
    NFT_REG_2 = 2,
    NFT_REG_3 = 3,
    NFT_REG_4 = 4,
    __NFT_REG_MAX = 5,
    NFT_REG32_00 = 8,
    MFT_REG32_01 = 9,
    NFT_REG32_02 = 10,
    NFT_REG32_03 = 11,
    NFT_REG32_04 = 12,
    NFT_REG32_05 = 13,
    NFT_REG32_06 = 14,
    NFT_REG32_07 = 15,
    NFT_REG32_08 = 16,
    NFT_REG32_09 = 17,
    NFT_REG32_10 = 18,
    NFT_REG32_11 = 19,
    NFT_REG32_12 = 20,
    NFT_REG32_13 = 21,
    NFT_REG32_14 = 22,
    NFT_REG32_15 = 23,
}
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum nft_verdicts {
    NFT_CONTINUE = -1,
    NFT_BREAK = -2,
    NFT_JUMP = -3,
    NFT_GOTO = -4,
    NFT_RETURN = -5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nf_tables_msg_types {
    NFT_MSG_NEWTABLE = 0,
    NFT_MSG_GETTABLE = 1,
    NFT_MSG_DELTABLE = 2,
    NFT_MSG_NEWCHAIN = 3,
    NFT_MSG_GETCHAIN = 4,
    NFT_MSG_DELCHAIN = 5,
    NFT_MSG_NEWRULE = 6,
    NFT_MSG_GETRULE = 7,
    NFT_MSG_DELRULE = 8,
    NFT_MSG_NEWSET = 9,
    NFT_MSG_GETSET = 10,
    NFT_MSG_DELSET = 11,
    NFT_MSG_NEWSETELEM = 12,
    NFT_MSG_GETSETELEM = 13,
    NFT_MSG_DELSETELEM = 14,
    NFT_MSG_NEWGEN = 15,
    NFT_MSG_GETGEN = 16,
    NFT_MSG_TRACE = 17,
    NFT_MSG_MAX = 18,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_list_attributes {
    NFTA_LIST_UNPEC = 0,
    NFTA_LIST_ELEM = 1,
    __NFTA_LIST_MAX = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_hook_attributes {
    NFTA_HOOK_UNSPEC = 0,
    NFTA_HOOK_HOOKNUM = 1,
    NFTA_HOOK_PRIORITY = 2,
    NFTA_HOOK_DEV = 3,
    __NFTA_HOOK_MAX = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_table_flags { NFT_TABLE_F_DORMANT = 1, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_table_attributes {
    NFTA_TABLE_UNSPEC = 0,
    NFTA_TABLE_NAME = 1,
    NFTA_TABLE_FLAGS = 2,
    NFTA_TABLE_USE = 3,
    __NFTA_TABLE_MAX = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_chain_attributes {
    NFTA_CHAIN_UNSPEC = 0,
    NFTA_CHAIN_TABLE = 1,
    NFTA_CHAIN_HANDLE = 2,
    NFTA_CHAIN_NAME = 3,
    NFTA_CHAIN_HOOK = 4,
    NFTA_CHAIN_POLICY = 5,
    NFTA_CHAIN_USE = 6,
    NFTA_CHAIN_TYPE = 7,
    NFTA_CHAIN_COUNTERS = 8,
    NFTA_CHAIN_PAD = 9,
    __NFTA_CHAIN_MAX = 10,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_rule_attributes {
    NFTA_RULE_UNSPEC = 0,
    NFTA_RULE_TABLE = 1,
    NFTA_RULE_CHAIN = 2,
    NFTA_RULE_HANDLE = 3,
    NFTA_RULE_EXPRESSIONS = 4,
    NFTA_RULE_COMPAT = 5,
    NFTA_RULE_POSITION = 6,
    NFTA_RULE_USERDATA = 7,
    NFTA_RULE_PAD = 8,
    __NFTA_RULE_MAX = 9,
}
pub const NFT_RULE_COMPAT_F_MASK: nft_rule_compat_flags =
    nft_rule_compat_flags::NFT_RULE_COMPAT_F_INV;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_rule_compat_flags { NFT_RULE_COMPAT_F_INV = 2, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_rule_compat_attributes {
    NFTA_RULE_COMPAT_UNSPEC = 0,
    NFTA_RULE_COMPAT_PROTO = 1,
    NFTA_RULE_COMPAT_FLAGS = 2,
    __NFTA_RULE_COMPAT_MAX = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_set_flags {
    NFT_SET_ANONYMOUS = 1,
    NFT_SET_CONSTANT = 2,
    NFT_SET_INTERVAL = 4,
    NFT_SET_MAP = 8,
    NFT_SET_TIMEOUT = 16,
    NFT_SET_EVAL = 32,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_set_policies {
    NFT_SET_POL_PERFORMANCE = 0,
    NFT_SET_POL_MEMORY = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_set_desc_attributes {
    NFTA_SET_DESC_UNSPEC = 0,
    NFTA_SET_DESC_SIZE = 1,
    __NFTA_SET_DESC_MAX = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_set_attributes {
    NFTA_SET_UNSPEC = 0,
    NFTA_SET_TABLE = 1,
    NFTA_SET_NAME = 2,
    NFTA_SET_FLAGS = 3,
    NFTA_SET_KEY_TYPE = 4,
    NFTA_SET_KEY_LEN = 5,
    NFTA_SET_DATA_TYPE = 6,
    NFTA_SET_DATA_LEN = 7,
    NFTA_SET_POLICY = 8,
    NFTA_SET_DESC = 9,
    NFTA_SET_ID = 10,
    NFTA_SET_TIMEOUT = 11,
    NFTA_SET_GC_INTERVAL = 12,
    NFTA_SET_USERDATA = 13,
    NFTA_SET_PAD = 14,
    __NFTA_SET_MAX = 15,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_set_elem_flags { NFT_SET_ELEM_INTERVAL_END = 1, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_set_elem_attributes {
    NFTA_SET_ELEM_UNSPEC = 0,
    NFTA_SET_ELEM_KEY = 1,
    NFTA_SET_ELEM_DATA = 2,
    NFTA_SET_ELEM_FLAGS = 3,
    NFTA_SET_ELEM_TIMEOUT = 4,
    NFTA_SET_ELEM_EXPIRATION = 5,
    NFTA_SET_ELEM_USERDATA = 6,
    NFTA_SET_ELEM_EXPR = 7,
    NFTA_SET_ELEM_PAD = 8,
    __NFTA_SET_ELEM_MAX = 9,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_set_elem_list_attributes {
    NFTA_SET_ELEM_LIST_UNSPEC = 0,
    NFTA_SET_ELEM_LIST_TABLE = 1,
    NFTA_SET_ELEM_LIST_SET = 2,
    NFTA_SET_ELEM_LIST_ELEMENTS = 3,
    NFTA_SET_ELEM_LIST_SET_ID = 4,
    __NFTA_SET_ELEM_LIST_MAX = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_data_types { NFT_DATA_VALUE = 0, NFT_DATA_VERDICT = 4294967040, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_data_attributes {
    NFTA_DATA_UNSPEC = 0,
    NFTA_DATA_VALUE = 1,
    NFTA_DATA_VERDICT = 2,
    __NFTA_DATA_MAX = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_verdict_attributes {
    NFTA_VERDICT_UNSPEC = 0,
    NFTA_VERDICT_CODE = 1,
    NFTA_VERDICT_CHAIN = 2,
    __NFTA_VERDICT_MAX = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_expr_attributes {
    NFTA_EXPR_UNSPEC = 0,
    NFTA_EXPR_NAME = 1,
    NFTA_EXPR_DATA = 2,
    __NFTA_EXPR_MAX = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_immediate_attributes {
    NFTA_IMMEDIATE_UNSPEC = 0,
    NFTA_IMMEDIATE_DREG = 1,
    NFTA_IMMEDIATE_DATA = 2,
    __NFTA_IMMEDIATE_MAX = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_bitwise_attributes {
    NFTA_BITWISE_UNSPEC = 0,
    NFTA_BITWISE_SREG = 1,
    NFTA_BITWISE_DREG = 2,
    NFTA_BITWISE_LEN = 3,
    NFTA_BITWISE_MASK = 4,
    NFTA_BITWISE_XOR = 5,
    __NFTA_BITWISE_MAX = 6,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_byteorder_ops { NFT_BYTEORDER_NTOH = 0, NFT_BYTEORDER_HTON = 1, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_byteorder_attributes {
    NFTA_BYTEORDER_UNSPEC = 0,
    NFTA_BYTEORDER_SREG = 1,
    NFTA_BYTEORDER_DREG = 2,
    NFTA_BYTEORDER_OP = 3,
    NFTA_BYTEORDER_LEN = 4,
    NFTA_BYTEORDER_SIZE = 5,
    __NFTA_BYTEORDER_MAX = 6,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_cmp_ops {
    NFT_CMP_EQ = 0,
    NFT_CMP_NEQ = 1,
    NFT_CMP_LT = 2,
    NFT_CMP_LTE = 3,
    NFT_CMP_GT = 4,
    NFT_CMP_GTE = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_cmp_attributes {
    NFTA_CMP_UNSPEC = 0,
    NFTA_CMP_SREG = 1,
    NFTA_CMP_OP = 2,
    NFTA_CMP_DATA = 3,
    __NFTA_CMP_MAX = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_lookup_attributes {
    NFTA_LOOKUP_UNSPEC = 0,
    NFTA_LOOKUP_SET = 1,
    NFTA_LOOKUP_SREG = 2,
    NFTA_LOOKUP_DREG = 3,
    NFTA_LOOKUP_SET_ID = 4,
    __NFTA_LOOKUP_MAX = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_dynset_ops { NFT_DYNSET_OP_ADD = 0, NFT_DYNSET_OP_UPDATE = 1, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_dynset_attributes {
    NFTA_DYNSET_UNSPEC = 0,
    NFTA_DYNSET_SET_NAME = 1,
    NFTA_DYNSET_SET_ID = 2,
    NFTA_DYNSET_OP = 3,
    NFTA_DYNSET_SREG_KEY = 4,
    NFTA_DYNSET_SREG_DATA = 5,
    NFTA_DYNSET_TIMEOUT = 6,
    NFTA_DYNSET_EXPR = 7,
    NFTA_DYNSET_PAD = 8,
    __NFTA_DYNSET_MAX = 9,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_payload_bases {
    NFT_PAYLOAD_LL_HEADER = 0,
    NFT_PAYLOAD_NETWORK_HEADER = 1,
    NFT_PAYLOAD_TRANSPORT_HEADER = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_payload_csum_types {
    NFT_PAYLOAD_CSUM_NONE = 0,
    NFT_PAYLOAD_CSUM_INET = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_payload_attributes {
    NFTA_PAYLOAD_UNSPEC = 0,
    NFTA_PAYLOAD_DREG = 1,
    NFTA_PAYLOAD_BASE = 2,
    NFTA_PAYLOAD_OFFSET = 3,
    NFTA_PAYLOAD_LEN = 4,
    NFTA_PAYLOAD_SREG = 5,
    NFTA_PAYLOAD_CSUM_TYPE = 6,
    NFTA_PAYLOAD_CSUM_OFFSET = 7,
    __NFTA_PAYLOAD_MAX = 8,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_exthdr_attributes {
    NFTA_EXTHDR_UNSPEC = 0,
    NFTA_EXTHDR_DREG = 1,
    NFTA_EXTHDR_TYPE = 2,
    NFTA_EXTHDR_OFFSET = 3,
    NFTA_EXTHDR_LEN = 4,
    __NFTA_EXTHDR_MAX = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_meta_keys {
    NFT_META_LEN = 0,
    NFT_META_PROTOCOL = 1,
    NFT_META_PRIORITY = 2,
    NFT_META_MARK = 3,
    NFT_META_IIF = 4,
    NFT_META_OIF = 5,
    NFT_META_IIFNAME = 6,
    NFT_META_OIFNAME = 7,
    NFT_META_IIFTYPE = 8,
    NFT_META_OIFTYPE = 9,
    NFT_META_SKUID = 10,
    NFT_META_SKGID = 11,
    NFT_META_NFTRACE = 12,
    NFT_META_RTCLASSID = 13,
    NFT_META_SECMARK = 14,
    NFT_META_NFPROTO = 15,
    NFT_META_L4PROTO = 16,
    NFT_META_BRI_IIFNAME = 17,
    NFT_META_BRI_OIFNAME = 18,
    NFT_META_PKTTYPE = 19,
    NFT_META_CPU = 20,
    NFT_META_IIFGROUP = 21,
    NFT_META_OIFGROUP = 22,
    NFT_META_CGROUP = 23,
    NFT_META_PRANDOM = 24,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_meta_attributes {
    NFTA_META_UNSPEC = 0,
    NFTA_META_DREG = 1,
    NFTA_META_KEY = 2,
    NFTA_META_SREG = 3,
    __NFTA_META_MAX = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_ct_keys {
    NFT_CT_STATE = 0,
    NFT_CT_DIRECTION = 1,
    NFT_CT_STATUS = 2,
    NFT_CT_MARK = 3,
    NFT_CT_SECMARK = 4,
    NFT_CT_EXPIRATION = 5,
    NFT_CT_HELPER = 6,
    NFT_CT_L3PROTOCOL = 7,
    NFT_CT_SRC = 8,
    NFT_CT_DST = 9,
    NFT_CT_PROTOCOL = 10,
    NFT_CT_PROTO_SRC = 11,
    NFT_CT_PROTO_DST = 12,
    NFT_CT_LABELS = 13,
    NFT_CT_PKTS = 14,
    NFT_CT_BYTES = 15,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_ct_attributes {
    NFTA_CT_UNSPEC = 0,
    NFTA_CT_DREG = 1,
    NFTA_CT_KEY = 2,
    NFTA_CT_DIRECTION = 3,
    NFTA_CT_SREG = 4,
    __NFTA_CT_MAX = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_limit_type { NFT_LIMIT_PKTS = 0, NFT_LIMIT_PKT_BYTES = 1, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_limit_flags { NFT_LIMIT_F_INV = 1, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_limit_attributes {
    NFTA_LIMIT_UNSPEC = 0,
    NFTA_LIMIT_RATE = 1,
    NFTA_LIMIT_UNIT = 2,
    NFTA_LIMIT_BURST = 3,
    NFTA_LIMIT_TYPE = 4,
    NFTA_LIMIT_FLAGS = 5,
    NFTA_LIMIT_PAD = 6,
    __NFTA_LIMIT_MAX = 7,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_counter_attributes {
    NFTA_COUNTER_UNSPEC = 0,
    NFTA_COUNTER_BYTES = 1,
    NFTA_COUNTER_PACKETS = 2,
    NFTA_COUNTER_PAD = 3,
    __NFTA_COUNTER_MAX = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_log_attributes {
    NFTA_LOG_UNSPEC = 0,
    NFTA_LOG_GROUP = 1,
    NFTA_LOG_PREFIX = 2,
    NFTA_LOG_SNAPLEN = 3,
    NFTA_LOG_QTHRESHOLD = 4,
    NFTA_LOG_LEVEL = 5,
    NFTA_LOG_FLAGS = 6,
    __NFTA_LOG_MAX = 7,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_queue_attributes {
    NFTA_QUEUE_UNSPEC = 0,
    NFTA_QUEUE_NUM = 1,
    NFTA_QUEUE_TOTAL = 2,
    NFTA_QUEUE_FLAGS = 3,
    __NFTA_QUEUE_MAX = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_reject_types {
    NFT_REJECT_ICMP_UNREACH = 0,
    NFT_REJECT_TCP_RST = 1,
    NFT_REJECT_ICMPX_UNREACH = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_reject_inet_code {
    NFT_REJECT_ICMPX_NO_ROUTE = 0,
    NFT_REJECT_ICMPX_PORT_UNREACH = 1,
    NFT_REJECT_ICMPX_HOST_UNREACH = 2,
    NFT_REJECT_ICMPX_ADMIN_PROHIBITED = 3,
    __NFT_REJECT_ICMPX_MAX = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_reject_attributes {
    NFTA_REJECT_UNSPEC = 0,
    NFTA_REJECT_TYPE = 1,
    NFTA_REJECT_ICMP_CODE = 2,
    __NFTA_REJECT_MAX = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_nat_types { NFT_NAT_SNAT = 0, NFT_NAT_DNAT = 1, }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_nat_attributes {
    NFTA_NAT_UNSPEC = 0,
    NFTA_NAT_TYPE = 1,
    NFTA_NAT_FAMILY = 2,
    NFTA_NAT_REG_ADDR_MIN = 3,
    NFTA_NAT_REG_ADDR_MAX = 4,
    NFTA_NAT_REG_PROTO_MIN = 5,
    NFTA_NAT_REG_PROTO_MAX = 6,
    NFTA_NAT_FLAGS = 7,
    __NFTA_NAT_MAX = 8,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_masq_attributes {
    NFTA_MASQ_UNSPEC = 0,
    NFTA_MASQ_FLAGS = 1,
    NFTA_MASQ_REG_PROTO_MIN = 2,
    NFTA_MASQ_REG_PROTO_MAX = 3,
    __NFTA_MASQ_MAX = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_redir_attributes {
    NFTA_REDIR_UNSPEC = 0,
    NFTA_REDIR_REG_PROTO_MIN = 1,
    NFTA_REDIR_REG_PROTO_MAX = 2,
    NFTA_REDIR_FLAGS = 3,
    __NFTA_REDIR_MAX = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_dup_attributes {
    NFTA_DUP_UNSPEC = 0,
    NFTA_DUP_SREG_ADDR = 1,
    NFTA_DUP_SREG_DEV = 2,
    __NFTA_DUP_MAX = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_fwd_attributes {
    NFTA_FWD_UNSPEC = 0,
    NFTA_FWD_SREG_DEV = 1,
    __NFTA_FWD_MAX = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_gen_attributes {
    NFTA_GEN_UNSPEC = 0,
    NFTA_GEN_ID = 1,
    __NFTA_GEN_MAX = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_trace_attibutes {
    NFTA_TRACE_UNSPEC = 0,
    NFTA_TRACE_TABLE = 1,
    NFTA_TRACE_CHAIN = 2,
    NFTA_TRACE_RULE_HANDLE = 3,
    NFTA_TRACE_TYPE = 4,
    NFTA_TRACE_VERDICT = 5,
    NFTA_TRACE_ID = 6,
    NFTA_TRACE_LL_HEADER = 7,
    NFTA_TRACE_NETWORK_HEADER = 8,
    NFTA_TRACE_TRANSPORT_HEADER = 9,
    NFTA_TRACE_IIF = 10,
    NFTA_TRACE_IIFTYPE = 11,
    NFTA_TRACE_OIF = 12,
    NFTA_TRACE_OIFTYPE = 13,
    NFTA_TRACE_MARK = 14,
    NFTA_TRACE_NFPROTO = 15,
    NFTA_TRACE_POLICY = 16,
    NFTA_TRACE_PAD = 17,
    __NFTA_TRACE_MAX = 18,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nft_trace_types {
    NFT_TRACETYPE_UNSPEC = 0,
    NFT_TRACETYPE_POLICY = 1,
    NFT_TRACETYPE_RETURN = 2,
    NFT_TRACETYPE_RULE = 3,
    __NFT_TRACETYPE_MAX = 4,
}