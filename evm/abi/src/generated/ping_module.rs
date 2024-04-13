pub use ping_module::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod ping_module {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("admin"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("dispatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dispatch"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("request"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct PostRequest"),
                                ),
                            },],
                            outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dispatch"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("request"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct GetRequest"),
                                ),
                            },],
                            outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dispatchPostResponse"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dispatchPostResponse",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("response"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PostResponse"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dispatchToParachain"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dispatchToParachain",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_paraId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("host"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("host"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onAccept"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onAccept"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("request"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PostRequest"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onGetResponse"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onGetResponse"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct GetResponse"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onGetTimeout"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onGetTimeout"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct GetRequest"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onPostRequestTimeout"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onPostRequestTimeout",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PostRequest"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onPostResponse"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onPostResponse"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PostResponse"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onPostResponseTimeout"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onPostResponseTimeout",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PostResponse"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ping"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ping"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pingMessage"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PingMessage"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setIsmpHost"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setIsmpHost"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("hostAddr"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("GetResponseReceived"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("GetResponseReceived",),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GetTimeoutReceived"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("GetTimeoutReceived"),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MessageDispatched"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MessageDispatched"),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PostReceived"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PostReceived"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("message"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PostRequestTimeoutReceived"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PostRequestTimeoutReceived",),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PostResponseReceived"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PostResponseReceived",),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PostResponseTimeoutReceived"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PostResponseTimeoutReceived",),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ExecutionFailed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotIsmpHost"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotIsmpHost"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PINGMODULE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x18\xA18\x03\x80a\x18\xA1\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x18\x0E\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\xBC\r\xD4G\x11a\0\x8CW\x80c\xDE\xAET\xF5\x11a\0fW\x80c\xDE\xAET\xF5\x14a\x01\x94W\x80c\xEA\xEE\x1C\xAA\x14a\x01\xA7W\x80c\xF47\xBCY\x14a\x01\xBAW\x80c\xFE\xFF\x7F\xA8\x14a\x01\xD5W`\0\x80\xFD[\x80c\xBC\r\xD4G\x14a\x01[W\x80c\xC4\x92\xE4&\x14a\x01nW\x80c\xD5\xF6\xEE\xFD\x14a\x01\x81W`\0\x80\xFD[\x80c\x0B\xC3{\xAB\x14a\0\xD4W\x80c\x0E\x83$\xA2\x14a\0\xE9W\x80cJi.\x06\x14a\0\xFCW\x80cM\r\x9C;\x14a\x01\x0FW\x80cp\xC5GO\x14a\x015W\x80cr5N\x9B\x14a\x01HW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x0EkV[a\x01\xE8V[\0[a\0\xE7a\0\xF76`\x04a\x0F:V[a\x02?V[a\0\xE7a\x01\n6`\x04a\x0FWV[a\x02\x8CV[a\x01\"a\x01\x1D6`\x04a\x0EkV[a\x04@V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\"a\x01C6`\x04a\x10\x01V[a\x04\xF0V[a\0\xE7a\x01V6`\x04a\x105V[a\x070V[a\0\xE7a\x01i6`\x04a\x10\x01V[a\x08JV[a\0\xE7a\x01|6`\x04a\x11\xE6V[a\x08\xA1V[a\x01\"a\x01\x8F6`\x04a\x11\xE6V[a\x08\xF8V[a\0\xE7a\x01\xA26`\x04a\x12\x1AV[a\tmV[a\0\xE7a\x01\xB56`\x04a\x10\x01V[a\t\xC4V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01,V[a\0\xE7a\x01\xE36`\x04a\x0EkV[a\n-V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\x13W`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7Fhv\xFA>\xCC}\x82\x1F!]\x82\x12B\xCB\xBE\x1F\x0E0\xA0\n\x85\xC2\"\xD6\x92\xA7\x96\x8F\xD3\xAF\xF1\x0B\x90`\0\x90\xA1PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02jW`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0[\x81``\x01Q\x81\x10\x15a\x04<W`\0`@Q\x80`\xC0\x01`@R\x80\x84`\0\x01Q\x81R` \x01\x84` \x01Q`@Q` \x01a\x02\xDF\x91\x90``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R`\x14\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81R` \x01`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF47\xBCY`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03l\x91\x90\x81\x01\x90a\x13\x9BV[`@Q` \x01a\x03|\x91\x90a\x14\x11V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x90\x82R\x85\x81\x01Q`\x01`\x01`@\x1B\x03\x16` \x83\x01R`\x80\x86\x01Q\x82\x82\x01R2``\x90\x92\x01\x91\x90\x91R`\0T\x90Qc\xB8\xF3\xE8\xF5`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB8\xF3\xE8\xF5\x90a\x03\xE4\x90\x84\x90`\x04\x01a\x14pV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04'\x91\x90a\x15\x04V[PP\x80\x80a\x044\x90a\x153V[\x91PPa\x02\x8FV[PPV[`@\x80Q`\xA0\x81\x01\x82R\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x82\x82\x01Q`\x01`\x01`@\x1B\x03\x16\x81\x83\x01R`\0``\x82\x01\x81\x90R2`\x80\x83\x01R\x80T\x92Qc\x94H\x08\x05`\xE0\x1B\x81R\x90\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\x94H\x08\x05\x90a\x04\xA6\x90\x84\x90`\x04\x01a\x15LV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE9\x91\x90a\x15\x04V[\x93\x92PPPV[`\0\x80T`@\x80Qcd\x1Dr\x9D`\xE0\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cd\x1Dr\x9D\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05^\x91\x90a\x15\x04V[\x90P`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cdxF\xA5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD5\x91\x90a\x16}V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86`\xC0\x01QQ\x85a\x05\xF6\x91\x90a\x16\x9AV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06n\x91\x90a\x16\xB1V[P`@\x80Q`\xC0\x80\x82\x01\x83R` \x80\x87\x01Q\x83R`\x80\x80\x88\x01Q\x91\x84\x01\x91\x90\x91R\x90\x86\x01Q\x82\x84\x01R`\xA0\x80\x87\x01Q`\x01`\x01`@\x1B\x03\x16``\x84\x01R`\0\x91\x83\x01\x82\x90R2\x90\x83\x01RT\x91Qc\xB8\xF3\xE8\xF5`\xE0\x1B\x81R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB8\xF3\xE8\xF5\x90a\x06\xE5\x90\x84\x90`\x04\x01a\x14pV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07(\x91\x90a\x15\x04V[\x94\x93PPPPV[`\0`@Q\x80`\xC0\x01`@R\x80a\x07F\x84a\n\x84V[\x81R` \x01`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g\x1A\\\xDB\\\x0BX\\\xDD`\xC2\x1B\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mhello from evm`\x90\x1B\x81RP\x81R` \x01`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0\x81R` \x012`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90P`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xB8\xF3\xE8\xF5\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\x02\x91\x90a\x14pV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08E\x91\x90a\x15\x04V[PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08uW`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\xBB\xF4\x8AR\xB8>\xBC=\x9E9\xF0\x92\xA8\xB9\xB7\xE5o\x1D\xD0\xDCC\x8B\xEF@\xDC}\x92\x99Bp\xA5\x9F\x90`\0\x90\xA1PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xCCW`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x83\xE6 %\xE4\xBCXu\x16\xD0\xBC1^2\x9E\xAC\x0Cf6(T\xFE\xB7\xCDA5\xEF\x81C\xBA\x15\xF9\x90`\0\x90\xA1PV[`@\x80Q`\xA0\x80\x82\x01\x83R` \x80\x85\x01Q\x83R`\xC0\x85\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91R\x90\x84\x01Q\x82\x84\x01R`\x80\x80\x85\x01Q\x90\x91\x16``\x83\x01R3\x90\x82\x01R`\0\x80T\x92Qc47VA`\xE2\x1B\x81R\x90\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\xDDY\x04\x90a\x04\xA6\x90\x84\x90`\x04\x01a\x16\xD3V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\x98W`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\xCB\xCB\xCAfM\xFE\xB9$\xCC\xD8P\xA0\x08h\x13\x0B\xFB\x1D\xF1W\t\x9A\x06\xF9)h\"\xCB{\xC3\xAD\x01\x90`\0\x90\xA1PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xEFW`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xFB\x08{?\xFB\xBB\x0F\xC9\"\xDC\xCF\x87%\x08g\x1Av\x05\x85\x94#\xEB\x90\xEB\x01LV\xFD\xBA\x14\x84\xDC\x81`\xC0\x01Q`@Qa\n\"\x91\x90a\x17\x96V[`@Q\x80\x91\x03\x90\xA1PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\nXW`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\xD7\xDC\x99\xAF\xB6\xC309\xCE\xA4PZ\x9E,\xAB4q\xD3Y\xCE\xBE\x02\x1E\xC1'\xDC\x94\xDD\xD3Y\xD3\xC5\x90`\0\x90\xA1PV[``a\n\x8F\x82a\n\xB5V[`@Q` \x01a\n\x9F\x91\x90a\x17\xA9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\0a\n\xC2\x83a\x0BHV[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xE1Wa\n\xE1a\x0C!V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\x0BW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0B\x15W[P\x93\x92PPPV[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x0B\x87Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x0B\xB3Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x0B\xD1Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x0B\xE9Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x0B\xFDWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x0C\x0FW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x0C\x1BW`\x01\x01[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0CYWa\x0CYa\x0C!V[`@R\x90V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0CYWa\x0CYa\x0C!V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0CYWa\x0CYa\x0C!V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C\xCBWa\x0C\xCBa\x0C!V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0C\xECWa\x0C\xECa\x0C!V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\r\x0BW`\0\x80\xFD[\x815a\r\x1Ea\r\x19\x82a\x0C\xD3V[a\x0C\xA3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\r3W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\rgW`\0\x80\xFD[\x91\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a\r~W`\0\x80\xFD[a\r\x86a\x0C7V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\r\x9FW`\0\x80\xFD[a\r\xAB\x85\x83\x86\x01a\x0C\xFAV[\x83R` \x84\x015\x91P\x80\x82\x11\x15a\r\xC1W`\0\x80\xFD[a\r\xCD\x85\x83\x86\x01a\x0C\xFAV[` \x84\x01Ra\r\xDE`@\x85\x01a\rPV[`@\x84\x01R``\x84\x015\x91P\x80\x82\x11\x15a\r\xF7W`\0\x80\xFD[a\x0E\x03\x85\x83\x86\x01a\x0C\xFAV[``\x84\x01R`\x80\x84\x015\x91P\x80\x82\x11\x15a\x0E\x1CW`\0\x80\xFD[a\x0E(\x85\x83\x86\x01a\x0C\xFAV[`\x80\x84\x01Ra\x0E9`\xA0\x85\x01a\rPV[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15a\x0ERW`\0\x80\xFD[Pa\x0E_\x84\x82\x85\x01a\x0C\xFAV[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0E}W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E\x94W`\0\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15a\x0E\xA8W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\x0E\xC3Wa\x0E\xC3a\x0C!V[`@R\x825\x82\x81\x11\x15a\x0E\xD5W`\0\x80\xFD[a\x0E\xE1\x87\x82\x86\x01a\rlV[\x82RP` \x83\x015\x82\x81\x11\x15a\x0E\xF6W`\0\x80\xFD[a\x0F\x02\x87\x82\x86\x01a\x0C\xFAV[` \x83\x01RPa\x0F\x14`@\x84\x01a\rPV[`@\x82\x01R\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F7W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x0FLW`\0\x80\xFD[\x815a\x04\xE9\x81a\x0F\"V[`\0` \x82\x84\x03\x12\x15a\x0FiW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0F\x80W`\0\x80\xFD[\x90\x83\x01\x90`\xA0\x82\x86\x03\x12\x15a\x0F\x94W`\0\x80\xFD[a\x0F\x9Ca\x0C_V[\x825\x82\x81\x11\x15a\x0F\xABW`\0\x80\xFD[a\x0F\xB7\x87\x82\x86\x01a\x0C\xFAV[\x82RP` \x83\x015\x91Pa\x0F\xCA\x82a\x0F\"V[\x81` \x82\x01Ra\x0F\xDC`@\x84\x01a\rPV[`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x10\x13W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10)W`\0\x80\xFD[a\x07(\x84\x82\x85\x01a\rlV[`\0` \x82\x84\x03\x12\x15a\x10GW`\0\x80\xFD[P5\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x10gWa\x10ga\x0C!V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x10\x82W`\0\x80\xFD[\x815` a\x10\x92a\r\x19\x83a\x10NV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x10\xB1W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x10\xF0W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xD4W`\0\x80\x81\xFD[a\x10\xE2\x89\x86\x83\x8B\x01\x01a\x0C\xFAV[\x84RP\x91\x83\x01\x91\x83\x01a\x10\xB5V[P\x96\x95PPPPPPV[`\0`\xE0\x82\x84\x03\x12\x15a\x11\rW`\0\x80\xFD[a\x11\x15a\x0C7V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11.W`\0\x80\xFD[a\x11:\x85\x83\x86\x01a\x0C\xFAV[\x83R` \x84\x015\x91P\x80\x82\x11\x15a\x11PW`\0\x80\xFD[a\x11\\\x85\x83\x86\x01a\x0C\xFAV[` \x84\x01Ra\x11m`@\x85\x01a\rPV[`@\x84\x01R``\x84\x015\x91P\x80\x82\x11\x15a\x11\x86W`\0\x80\xFD[a\x11\x92\x85\x83\x86\x01a\x0C\xFAV[``\x84\x01Ra\x11\xA3`\x80\x85\x01a\rPV[`\x80\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15a\x11\xBCW`\0\x80\xFD[Pa\x11\xC9\x84\x82\x85\x01a\x10qV[`\xA0\x83\x01RPa\x11\xDB`\xC0\x83\x01a\rPV[`\xC0\x82\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\xF8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\x0EW`\0\x80\xFD[a\x07(\x84\x82\x85\x01a\x10\xFBV[`\0` \x80\x83\x85\x03\x12\x15a\x12-W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x12DW`\0\x80\xFD[\x81\x85\x01\x91P`@\x80\x83\x88\x03\x12\x15a\x12ZW`\0\x80\xFD[a\x12ba\x0C\x81V[\x835\x83\x81\x11\x15a\x12qW`\0\x80\xFD[a\x12}\x89\x82\x87\x01a\x10\xFBV[\x82RP\x84\x84\x015\x83\x81\x11\x15a\x12\x91W`\0\x80\xFD[\x80\x85\x01\x94PP\x87`\x1F\x85\x01\x12a\x12\xA6W`\0\x80\xFD[\x835a\x12\xB4a\r\x19\x82a\x10NV[\x81\x81R`\x05\x91\x90\x91\x1B\x85\x01\x86\x01\x90\x86\x81\x01\x90\x8A\x83\x11\x15a\x12\xD3W`\0\x80\xFD[\x87\x87\x01[\x83\x81\x10\x15a\x13cW\x805\x87\x81\x11\x15a\x12\xEFW`\0\x80\x81\xFD[\x88\x01\x80\x8D\x03`\x1F\x19\x01\x87\x13\x15a\x13\x05W`\0\x80\x81\xFD[a\x13\ra\x0C\x81V[\x8A\x82\x015\x89\x81\x11\x15a\x13\x1FW`\0\x80\x81\xFD[a\x13-\x8F\x8D\x83\x86\x01\x01a\x0C\xFAV[\x82RP\x87\x82\x015\x89\x81\x11\x15a\x13BW`\0\x80\x81\xFD[a\x13P\x8F\x8D\x83\x86\x01\x01a\x0C\xFAV[\x82\x8D\x01RP\x84RP\x91\x88\x01\x91\x88\x01a\x12\xD7V[P\x96\x83\x01\x96\x90\x96RP\x97\x96PPPPPPPV[`\0[\x83\x81\x10\x15a\x13\x92W\x81\x81\x01Q\x83\x82\x01R` \x01a\x13zV[PP`\0\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\x13\xADW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xC3W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x13\xD4W`\0\x80\xFD[\x80Qa\x13\xE2a\r\x19\x82a\x0C\xD3V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x13\xF7W`\0\x80\xFD[a\x14\x08\x82` \x83\x01` \x86\x01a\x13wV[\x95\x94PPPPPV[j\x03C+ccy\x033\x93{i`\xAD\x1B\x81R`\0\x82Qa\x147\x81`\x0B\x85\x01` \x87\x01a\x13wV[\x91\x90\x91\x01`\x0B\x01\x92\x91PPV[`\0\x81Q\x80\x84Ra\x14\\\x81` \x86\x01` \x86\x01a\x13wV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0\x82Q`\xC0` \x84\x01Ra\x14\x8C`\xE0\x84\x01\x82a\x14DV[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra\x14\xAA\x83\x83a\x14DV[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa\x14\xC8\x82\x82a\x14DV[\x91PP`\x01`\x01`@\x1B\x03``\x85\x01Q\x16`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R`\x01\x80`\xA0\x1B\x03`\xA0\x85\x01Q\x16`\xC0\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\x16W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x15EWa\x15Ea\x15\x1DV[P`\x01\x01\x90V[` \x81R`\0\x82Q`\xA0` \x84\x01R\x80Q`\xE0`\xC0\x85\x01Ra\x15ra\x01\xA0\x85\x01\x82a\x14DV[\x90P` \x82\x01Q`\xBF\x19\x80\x86\x84\x03\x01`\xE0\x87\x01Ra\x15\x90\x83\x83a\x14DV[\x92P`\x01`\x01`@\x1B\x03`@\x85\x01Q\x16a\x01\0\x87\x01R``\x84\x01Q\x91P\x80\x86\x84\x03\x01a\x01 \x87\x01Ra\x15\xC2\x83\x83a\x14DV[\x92P`\x80\x84\x01Q\x91P\x80\x86\x84\x03\x01a\x01@\x87\x01Ra\x15\xE0\x83\x83a\x14DV[\x92P`\xA0\x84\x01Q\x91Pa\x15\xFFa\x01`\x87\x01\x83`\x01`\x01`@\x1B\x03\x16\x90RV[`\xC0\x84\x01Q\x93P\x80\x86\x84\x03\x01a\x01\x80\x87\x01RPPa\x16\x1D\x81\x83a\x14DV[\x91PP` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra\x16;\x82\x82a\x14DV[\x91PP`@\x84\x01Qa\x16X``\x85\x01\x82`\x01`\x01`@\x1B\x03\x16\x90RV[P``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Qa\x0B@`\xA0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\0` \x82\x84\x03\x12\x15a\x16\x8FW`\0\x80\xFD[\x81Qa\x04\xE9\x81a\x0F\"V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\x1BWa\x0C\x1Ba\x15\x1DV[`\0` \x82\x84\x03\x12\x15a\x16\xC3W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x04\xE9W`\0\x80\xFD[`\0` \x80\x83R\x83Q`\xA0\x82\x85\x01Ra\x16\xEF`\xC0\x85\x01\x82a\x14DV[\x90P`\x01`\x01`@\x1B\x03\x82\x86\x01Q\x16`@\x85\x01R`@\x85\x01Q`\x1F\x19\x80\x86\x84\x03\x01``\x87\x01R\x82\x82Q\x80\x85R\x85\x85\x01\x91P\x85\x81`\x05\x1B\x86\x01\x01\x86\x85\x01\x94P`\0[\x82\x81\x10\x15a\x17\\W\x84\x87\x83\x03\x01\x84Ra\x17J\x82\x87Qa\x14DV[\x95\x88\x01\x95\x93\x88\x01\x93\x91P`\x01\x01a\x170V[P``\x8A\x01Q`\x01`\x01`@\x1B\x03\x81\x16`\x80\x8B\x01R\x96P`\x80\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xA0\x8B\x01R\x96P\x99\x98PPPPPPPPPV[` \x81R`\0a\x04\xE9` \x83\x01\x84a\x14DV[fKUSAMA-`\xC8\x1B\x81R`\0\x82Qa\x17\xCB\x81`\x07\x85\x01` \x87\x01a\x13wV[\x91\x90\x91\x01`\x07\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x1C\xD1\xC4\x875x\x14 #\xB3zHZh\x8B\t\xED!\xD1\x80\xDE-\xF2!O{g2\xBCvb2dsolcC\0\x08\x11\x003";
    /// The bytecode of the contract.
    pub static PINGMODULE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\xBC\r\xD4G\x11a\0\x8CW\x80c\xDE\xAET\xF5\x11a\0fW\x80c\xDE\xAET\xF5\x14a\x01\x94W\x80c\xEA\xEE\x1C\xAA\x14a\x01\xA7W\x80c\xF47\xBCY\x14a\x01\xBAW\x80c\xFE\xFF\x7F\xA8\x14a\x01\xD5W`\0\x80\xFD[\x80c\xBC\r\xD4G\x14a\x01[W\x80c\xC4\x92\xE4&\x14a\x01nW\x80c\xD5\xF6\xEE\xFD\x14a\x01\x81W`\0\x80\xFD[\x80c\x0B\xC3{\xAB\x14a\0\xD4W\x80c\x0E\x83$\xA2\x14a\0\xE9W\x80cJi.\x06\x14a\0\xFCW\x80cM\r\x9C;\x14a\x01\x0FW\x80cp\xC5GO\x14a\x015W\x80cr5N\x9B\x14a\x01HW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x0EkV[a\x01\xE8V[\0[a\0\xE7a\0\xF76`\x04a\x0F:V[a\x02?V[a\0\xE7a\x01\n6`\x04a\x0FWV[a\x02\x8CV[a\x01\"a\x01\x1D6`\x04a\x0EkV[a\x04@V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\"a\x01C6`\x04a\x10\x01V[a\x04\xF0V[a\0\xE7a\x01V6`\x04a\x105V[a\x070V[a\0\xE7a\x01i6`\x04a\x10\x01V[a\x08JV[a\0\xE7a\x01|6`\x04a\x11\xE6V[a\x08\xA1V[a\x01\"a\x01\x8F6`\x04a\x11\xE6V[a\x08\xF8V[a\0\xE7a\x01\xA26`\x04a\x12\x1AV[a\tmV[a\0\xE7a\x01\xB56`\x04a\x10\x01V[a\t\xC4V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01,V[a\0\xE7a\x01\xE36`\x04a\x0EkV[a\n-V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\x13W`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7Fhv\xFA>\xCC}\x82\x1F!]\x82\x12B\xCB\xBE\x1F\x0E0\xA0\n\x85\xC2\"\xD6\x92\xA7\x96\x8F\xD3\xAF\xF1\x0B\x90`\0\x90\xA1PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02jW`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0[\x81``\x01Q\x81\x10\x15a\x04<W`\0`@Q\x80`\xC0\x01`@R\x80\x84`\0\x01Q\x81R` \x01\x84` \x01Q`@Q` \x01a\x02\xDF\x91\x90``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R`\x14\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81R` \x01`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF47\xBCY`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03l\x91\x90\x81\x01\x90a\x13\x9BV[`@Q` \x01a\x03|\x91\x90a\x14\x11V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x90\x82R\x85\x81\x01Q`\x01`\x01`@\x1B\x03\x16` \x83\x01R`\x80\x86\x01Q\x82\x82\x01R2``\x90\x92\x01\x91\x90\x91R`\0T\x90Qc\xB8\xF3\xE8\xF5`\xE0\x1B\x81R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB8\xF3\xE8\xF5\x90a\x03\xE4\x90\x84\x90`\x04\x01a\x14pV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04'\x91\x90a\x15\x04V[PP\x80\x80a\x044\x90a\x153V[\x91PPa\x02\x8FV[PPV[`@\x80Q`\xA0\x81\x01\x82R\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R\x82\x82\x01Q`\x01`\x01`@\x1B\x03\x16\x81\x83\x01R`\0``\x82\x01\x81\x90R2`\x80\x83\x01R\x80T\x92Qc\x94H\x08\x05`\xE0\x1B\x81R\x90\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\x94H\x08\x05\x90a\x04\xA6\x90\x84\x90`\x04\x01a\x15LV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE9\x91\x90a\x15\x04V[\x93\x92PPPV[`\0\x80T`@\x80Qcd\x1Dr\x9D`\xE0\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cd\x1Dr\x9D\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05^\x91\x90a\x15\x04V[\x90P`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cdxF\xA5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD5\x91\x90a\x16}V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86`\xC0\x01QQ\x85a\x05\xF6\x91\x90a\x16\x9AV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06n\x91\x90a\x16\xB1V[P`@\x80Q`\xC0\x80\x82\x01\x83R` \x80\x87\x01Q\x83R`\x80\x80\x88\x01Q\x91\x84\x01\x91\x90\x91R\x90\x86\x01Q\x82\x84\x01R`\xA0\x80\x87\x01Q`\x01`\x01`@\x1B\x03\x16``\x84\x01R`\0\x91\x83\x01\x82\x90R2\x90\x83\x01RT\x91Qc\xB8\xF3\xE8\xF5`\xE0\x1B\x81R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB8\xF3\xE8\xF5\x90a\x06\xE5\x90\x84\x90`\x04\x01a\x14pV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07(\x91\x90a\x15\x04V[\x94\x93PPPPV[`\0`@Q\x80`\xC0\x01`@R\x80a\x07F\x84a\n\x84V[\x81R` \x01`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g\x1A\\\xDB\\\x0BX\\\xDD`\xC2\x1B\x81RP\x81R` \x01`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mhello from evm`\x90\x1B\x81RP\x81R` \x01`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0\x81R` \x012`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90P`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xB8\xF3\xE8\xF5\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\x02\x91\x90a\x14pV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08E\x91\x90a\x15\x04V[PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08uW`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\xBB\xF4\x8AR\xB8>\xBC=\x9E9\xF0\x92\xA8\xB9\xB7\xE5o\x1D\xD0\xDCC\x8B\xEF@\xDC}\x92\x99Bp\xA5\x9F\x90`\0\x90\xA1PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xCCW`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\x83\xE6 %\xE4\xBCXu\x16\xD0\xBC1^2\x9E\xAC\x0Cf6(T\xFE\xB7\xCDA5\xEF\x81C\xBA\x15\xF9\x90`\0\x90\xA1PV[`@\x80Q`\xA0\x80\x82\x01\x83R` \x80\x85\x01Q\x83R`\xC0\x85\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x91\x84\x01\x91\x90\x91R\x90\x84\x01Q\x82\x84\x01R`\x80\x80\x85\x01Q\x90\x91\x16``\x83\x01R3\x90\x82\x01R`\0\x80T\x92Qc47VA`\xE2\x1B\x81R\x90\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD0\xDDY\x04\x90a\x04\xA6\x90\x84\x90`\x04\x01a\x16\xD3V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\x98W`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\xCB\xCB\xCAfM\xFE\xB9$\xCC\xD8P\xA0\x08h\x13\x0B\xFB\x1D\xF1W\t\x9A\x06\xF9)h\"\xCB{\xC3\xAD\x01\x90`\0\x90\xA1PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xEFW`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xFB\x08{?\xFB\xBB\x0F\xC9\"\xDC\xCF\x87%\x08g\x1Av\x05\x85\x94#\xEB\x90\xEB\x01LV\xFD\xBA\x14\x84\xDC\x81`\xC0\x01Q`@Qa\n\"\x91\x90a\x17\x96V[`@Q\x80\x91\x03\x90\xA1PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\nXW`@QcQ\xAB\x8D\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\xD7\xDC\x99\xAF\xB6\xC309\xCE\xA4PZ\x9E,\xAB4q\xD3Y\xCE\xBE\x02\x1E\xC1'\xDC\x94\xDD\xD3Y\xD3\xC5\x90`\0\x90\xA1PV[``a\n\x8F\x82a\n\xB5V[`@Q` \x01a\n\x9F\x91\x90a\x17\xA9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\0a\n\xC2\x83a\x0BHV[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xE1Wa\n\xE1a\x0C!V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\x0BW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0B\x15W[P\x93\x92PPPV[`\0\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x0B\x87Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x0B\xB3Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x0B\xD1Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x0B\xE9Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x0B\xFDWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x0C\x0FW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x0C\x1BW`\x01\x01[\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0CYWa\x0CYa\x0C!V[`@R\x90V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0CYWa\x0CYa\x0C!V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0CYWa\x0CYa\x0C!V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C\xCBWa\x0C\xCBa\x0C!V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0C\xECWa\x0C\xECa\x0C!V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\r\x0BW`\0\x80\xFD[\x815a\r\x1Ea\r\x19\x82a\x0C\xD3V[a\x0C\xA3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\r3W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\rgW`\0\x80\xFD[\x91\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a\r~W`\0\x80\xFD[a\r\x86a\x0C7V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\r\x9FW`\0\x80\xFD[a\r\xAB\x85\x83\x86\x01a\x0C\xFAV[\x83R` \x84\x015\x91P\x80\x82\x11\x15a\r\xC1W`\0\x80\xFD[a\r\xCD\x85\x83\x86\x01a\x0C\xFAV[` \x84\x01Ra\r\xDE`@\x85\x01a\rPV[`@\x84\x01R``\x84\x015\x91P\x80\x82\x11\x15a\r\xF7W`\0\x80\xFD[a\x0E\x03\x85\x83\x86\x01a\x0C\xFAV[``\x84\x01R`\x80\x84\x015\x91P\x80\x82\x11\x15a\x0E\x1CW`\0\x80\xFD[a\x0E(\x85\x83\x86\x01a\x0C\xFAV[`\x80\x84\x01Ra\x0E9`\xA0\x85\x01a\rPV[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15a\x0ERW`\0\x80\xFD[Pa\x0E_\x84\x82\x85\x01a\x0C\xFAV[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0E}W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E\x94W`\0\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15a\x0E\xA8W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\x0E\xC3Wa\x0E\xC3a\x0C!V[`@R\x825\x82\x81\x11\x15a\x0E\xD5W`\0\x80\xFD[a\x0E\xE1\x87\x82\x86\x01a\rlV[\x82RP` \x83\x015\x82\x81\x11\x15a\x0E\xF6W`\0\x80\xFD[a\x0F\x02\x87\x82\x86\x01a\x0C\xFAV[` \x83\x01RPa\x0F\x14`@\x84\x01a\rPV[`@\x82\x01R\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F7W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x0FLW`\0\x80\xFD[\x815a\x04\xE9\x81a\x0F\"V[`\0` \x82\x84\x03\x12\x15a\x0FiW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0F\x80W`\0\x80\xFD[\x90\x83\x01\x90`\xA0\x82\x86\x03\x12\x15a\x0F\x94W`\0\x80\xFD[a\x0F\x9Ca\x0C_V[\x825\x82\x81\x11\x15a\x0F\xABW`\0\x80\xFD[a\x0F\xB7\x87\x82\x86\x01a\x0C\xFAV[\x82RP` \x83\x015\x91Pa\x0F\xCA\x82a\x0F\"V[\x81` \x82\x01Ra\x0F\xDC`@\x84\x01a\rPV[`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x10\x13W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10)W`\0\x80\xFD[a\x07(\x84\x82\x85\x01a\rlV[`\0` \x82\x84\x03\x12\x15a\x10GW`\0\x80\xFD[P5\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x10gWa\x10ga\x0C!V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x10\x82W`\0\x80\xFD[\x815` a\x10\x92a\r\x19\x83a\x10NV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x10\xB1W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x10\xF0W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xD4W`\0\x80\x81\xFD[a\x10\xE2\x89\x86\x83\x8B\x01\x01a\x0C\xFAV[\x84RP\x91\x83\x01\x91\x83\x01a\x10\xB5V[P\x96\x95PPPPPPV[`\0`\xE0\x82\x84\x03\x12\x15a\x11\rW`\0\x80\xFD[a\x11\x15a\x0C7V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11.W`\0\x80\xFD[a\x11:\x85\x83\x86\x01a\x0C\xFAV[\x83R` \x84\x015\x91P\x80\x82\x11\x15a\x11PW`\0\x80\xFD[a\x11\\\x85\x83\x86\x01a\x0C\xFAV[` \x84\x01Ra\x11m`@\x85\x01a\rPV[`@\x84\x01R``\x84\x015\x91P\x80\x82\x11\x15a\x11\x86W`\0\x80\xFD[a\x11\x92\x85\x83\x86\x01a\x0C\xFAV[``\x84\x01Ra\x11\xA3`\x80\x85\x01a\rPV[`\x80\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15a\x11\xBCW`\0\x80\xFD[Pa\x11\xC9\x84\x82\x85\x01a\x10qV[`\xA0\x83\x01RPa\x11\xDB`\xC0\x83\x01a\rPV[`\xC0\x82\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x11\xF8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\x0EW`\0\x80\xFD[a\x07(\x84\x82\x85\x01a\x10\xFBV[`\0` \x80\x83\x85\x03\x12\x15a\x12-W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x12DW`\0\x80\xFD[\x81\x85\x01\x91P`@\x80\x83\x88\x03\x12\x15a\x12ZW`\0\x80\xFD[a\x12ba\x0C\x81V[\x835\x83\x81\x11\x15a\x12qW`\0\x80\xFD[a\x12}\x89\x82\x87\x01a\x10\xFBV[\x82RP\x84\x84\x015\x83\x81\x11\x15a\x12\x91W`\0\x80\xFD[\x80\x85\x01\x94PP\x87`\x1F\x85\x01\x12a\x12\xA6W`\0\x80\xFD[\x835a\x12\xB4a\r\x19\x82a\x10NV[\x81\x81R`\x05\x91\x90\x91\x1B\x85\x01\x86\x01\x90\x86\x81\x01\x90\x8A\x83\x11\x15a\x12\xD3W`\0\x80\xFD[\x87\x87\x01[\x83\x81\x10\x15a\x13cW\x805\x87\x81\x11\x15a\x12\xEFW`\0\x80\x81\xFD[\x88\x01\x80\x8D\x03`\x1F\x19\x01\x87\x13\x15a\x13\x05W`\0\x80\x81\xFD[a\x13\ra\x0C\x81V[\x8A\x82\x015\x89\x81\x11\x15a\x13\x1FW`\0\x80\x81\xFD[a\x13-\x8F\x8D\x83\x86\x01\x01a\x0C\xFAV[\x82RP\x87\x82\x015\x89\x81\x11\x15a\x13BW`\0\x80\x81\xFD[a\x13P\x8F\x8D\x83\x86\x01\x01a\x0C\xFAV[\x82\x8D\x01RP\x84RP\x91\x88\x01\x91\x88\x01a\x12\xD7V[P\x96\x83\x01\x96\x90\x96RP\x97\x96PPPPPPPV[`\0[\x83\x81\x10\x15a\x13\x92W\x81\x81\x01Q\x83\x82\x01R` \x01a\x13zV[PP`\0\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\x13\xADW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xC3W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x13\xD4W`\0\x80\xFD[\x80Qa\x13\xE2a\r\x19\x82a\x0C\xD3V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\x13\xF7W`\0\x80\xFD[a\x14\x08\x82` \x83\x01` \x86\x01a\x13wV[\x95\x94PPPPPV[j\x03C+ccy\x033\x93{i`\xAD\x1B\x81R`\0\x82Qa\x147\x81`\x0B\x85\x01` \x87\x01a\x13wV[\x91\x90\x91\x01`\x0B\x01\x92\x91PPV[`\0\x81Q\x80\x84Ra\x14\\\x81` \x86\x01` \x86\x01a\x13wV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0\x82Q`\xC0` \x84\x01Ra\x14\x8C`\xE0\x84\x01\x82a\x14DV[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01Ra\x14\xAA\x83\x83a\x14DV[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RPa\x14\xC8\x82\x82a\x14DV[\x91PP`\x01`\x01`@\x1B\x03``\x85\x01Q\x16`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R`\x01\x80`\xA0\x1B\x03`\xA0\x85\x01Q\x16`\xC0\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\x16W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x15EWa\x15Ea\x15\x1DV[P`\x01\x01\x90V[` \x81R`\0\x82Q`\xA0` \x84\x01R\x80Q`\xE0`\xC0\x85\x01Ra\x15ra\x01\xA0\x85\x01\x82a\x14DV[\x90P` \x82\x01Q`\xBF\x19\x80\x86\x84\x03\x01`\xE0\x87\x01Ra\x15\x90\x83\x83a\x14DV[\x92P`\x01`\x01`@\x1B\x03`@\x85\x01Q\x16a\x01\0\x87\x01R``\x84\x01Q\x91P\x80\x86\x84\x03\x01a\x01 \x87\x01Ra\x15\xC2\x83\x83a\x14DV[\x92P`\x80\x84\x01Q\x91P\x80\x86\x84\x03\x01a\x01@\x87\x01Ra\x15\xE0\x83\x83a\x14DV[\x92P`\xA0\x84\x01Q\x91Pa\x15\xFFa\x01`\x87\x01\x83`\x01`\x01`@\x1B\x03\x16\x90RV[`\xC0\x84\x01Q\x93P\x80\x86\x84\x03\x01a\x01\x80\x87\x01RPPa\x16\x1D\x81\x83a\x14DV[\x91PP` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra\x16;\x82\x82a\x14DV[\x91PP`@\x84\x01Qa\x16X``\x85\x01\x82`\x01`\x01`@\x1B\x03\x16\x90RV[P``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Qa\x0B@`\xA0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\0` \x82\x84\x03\x12\x15a\x16\x8FW`\0\x80\xFD[\x81Qa\x04\xE9\x81a\x0F\"V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\x1BWa\x0C\x1Ba\x15\x1DV[`\0` \x82\x84\x03\x12\x15a\x16\xC3W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x04\xE9W`\0\x80\xFD[`\0` \x80\x83R\x83Q`\xA0\x82\x85\x01Ra\x16\xEF`\xC0\x85\x01\x82a\x14DV[\x90P`\x01`\x01`@\x1B\x03\x82\x86\x01Q\x16`@\x85\x01R`@\x85\x01Q`\x1F\x19\x80\x86\x84\x03\x01``\x87\x01R\x82\x82Q\x80\x85R\x85\x85\x01\x91P\x85\x81`\x05\x1B\x86\x01\x01\x86\x85\x01\x94P`\0[\x82\x81\x10\x15a\x17\\W\x84\x87\x83\x03\x01\x84Ra\x17J\x82\x87Qa\x14DV[\x95\x88\x01\x95\x93\x88\x01\x93\x91P`\x01\x01a\x170V[P``\x8A\x01Q`\x01`\x01`@\x1B\x03\x81\x16`\x80\x8B\x01R\x96P`\x80\x8A\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xA0\x8B\x01R\x96P\x99\x98PPPPPPPPPV[` \x81R`\0a\x04\xE9` \x83\x01\x84a\x14DV[fKUSAMA-`\xC8\x1B\x81R`\0\x82Qa\x17\xCB\x81`\x07\x85\x01` \x87\x01a\x13wV[\x91\x90\x91\x01`\x07\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x1C\xD1\xC4\x875x\x14 #\xB3zHZh\x8B\t\xED!\xD1\x80\xDE-\xF2!O{g2\xBCvb2dsolcC\0\x08\x11\x003";
    /// The deployed bytecode of the contract.
    pub static PINGMODULE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PingModule<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PingModule<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PingModule<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PingModule<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PingModule<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PingModule)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PingModule<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(address.into(), PINGMODULE_ABI.clone(), client))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor
        /// arguments and sends it. Returns a new instance of a deployer that returns an
        /// instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                PINGMODULE_ABI.clone(),
                PINGMODULE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `dispatch` (0x70c5474f) function
        pub fn dispatch(
            &self,
            request: GetRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([112, 197, 71, 79], (request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dispatch` (0xd5f6eefd) function
        pub fn dispatch_with_request(
            &self,
            request: GetRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([213, 246, 238, 253], (request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dispatchPostResponse` (0x4d0d9c3b) function
        pub fn dispatch_post_response(
            &self,
            response: PostResponse,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([77, 13, 156, 59], (response,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dispatchToParachain` (0x72354e9b) function
        pub fn dispatch_to_parachain(
            &self,
            para_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 53, 78, 155], para_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `host` (0xf437bc59) function
        pub fn host(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([244, 55, 188, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onAccept` (0xeaee1caa) function
        pub fn on_accept(
            &self,
            request: PostRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 238, 28, 170], (request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onGetResponse` (0xdeae54f5) function
        pub fn on_get_response(
            &self,
            p0: GetResponse,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 174, 84, 245], (p0,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onGetTimeout` (0xc492e426) function
        pub fn on_get_timeout(
            &self,
            p0: GetRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 146, 228, 38], (p0,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onPostRequestTimeout` (0xbc0dd447) function
        pub fn on_post_request_timeout(
            &self,
            p0: PostRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 13, 212, 71], (p0,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onPostResponse` (0xfeff7fa8) function
        pub fn on_post_response(
            &self,
            p0: PostResponse,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 255, 127, 168], (p0,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onPostResponseTimeout` (0x0bc37bab) function
        pub fn on_post_response_timeout(
            &self,
            p0: PostResponse,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 195, 123, 171], (p0,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ping` (0x4a692e06) function
        pub fn ping(
            &self,
            ping_message: PingMessage,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 105, 46, 6], (ping_message,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setIsmpHost` (0x0e8324a2) function
        pub fn set_ismp_host(
            &self,
            host_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 131, 36, 162], host_addr)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `GetResponseReceived` event
        pub fn get_response_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GetResponseReceivedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `GetTimeoutReceived` event
        pub fn get_timeout_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GetTimeoutReceivedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `MessageDispatched` event
        pub fn message_dispatched_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MessageDispatchedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PostReceived` event
        pub fn post_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PostReceivedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PostRequestTimeoutReceived` event
        pub fn post_request_timeout_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PostRequestTimeoutReceivedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PostResponseReceived` event
        pub fn post_response_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PostResponseReceivedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PostResponseTimeoutReceived` event
        pub fn post_response_timeout_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PostResponseTimeoutReceivedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PingModuleEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for PingModule<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ExecutionFailed` with signature `ExecutionFailed()` and selector
    /// `0xacfdb444`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ExecutionFailed", abi = "ExecutionFailed()")]
    pub struct ExecutionFailed;
    ///Custom Error type `NotIsmpHost` with signature `NotIsmpHost()` and selector `0x51ab8de5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotIsmpHost", abi = "NotIsmpHost()")]
    pub struct NotIsmpHost;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PingModuleErrors {
        ExecutionFailed(ExecutionFailed),
        NotIsmpHost(NotIsmpHost),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PingModuleErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ExecutionFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecutionFailed(decoded));
            }
            if let Ok(decoded) = <NotIsmpHost as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotIsmpHost(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PingModuleErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ExecutionFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotIsmpHost(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PingModuleErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <ExecutionFailed as ::ethers::contract::EthError>::selector() =>
                    true,
                _ if selector == <NotIsmpHost as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PingModuleErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExecutionFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotIsmpHost(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PingModuleErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ExecutionFailed> for PingModuleErrors {
        fn from(value: ExecutionFailed) -> Self {
            Self::ExecutionFailed(value)
        }
    }
    impl ::core::convert::From<NotIsmpHost> for PingModuleErrors {
        fn from(value: NotIsmpHost) -> Self {
            Self::NotIsmpHost(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "GetResponseReceived", abi = "GetResponseReceived()")]
    pub struct GetResponseReceivedFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "GetTimeoutReceived", abi = "GetTimeoutReceived()")]
    pub struct GetTimeoutReceivedFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "MessageDispatched", abi = "MessageDispatched()")]
    pub struct MessageDispatchedFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PostReceived", abi = "PostReceived(string)")]
    pub struct PostReceivedFilter {
        pub message: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PostRequestTimeoutReceived", abi = "PostRequestTimeoutReceived()")]
    pub struct PostRequestTimeoutReceivedFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PostResponseReceived", abi = "PostResponseReceived()")]
    pub struct PostResponseReceivedFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PostResponseTimeoutReceived", abi = "PostResponseTimeoutReceived()")]
    pub struct PostResponseTimeoutReceivedFilter;
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PingModuleEvents {
        GetResponseReceivedFilter(GetResponseReceivedFilter),
        GetTimeoutReceivedFilter(GetTimeoutReceivedFilter),
        MessageDispatchedFilter(MessageDispatchedFilter),
        PostReceivedFilter(PostReceivedFilter),
        PostRequestTimeoutReceivedFilter(PostRequestTimeoutReceivedFilter),
        PostResponseReceivedFilter(PostResponseReceivedFilter),
        PostResponseTimeoutReceivedFilter(PostResponseTimeoutReceivedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PingModuleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = GetResponseReceivedFilter::decode_log(log) {
                return Ok(PingModuleEvents::GetResponseReceivedFilter(decoded));
            }
            if let Ok(decoded) = GetTimeoutReceivedFilter::decode_log(log) {
                return Ok(PingModuleEvents::GetTimeoutReceivedFilter(decoded));
            }
            if let Ok(decoded) = MessageDispatchedFilter::decode_log(log) {
                return Ok(PingModuleEvents::MessageDispatchedFilter(decoded));
            }
            if let Ok(decoded) = PostReceivedFilter::decode_log(log) {
                return Ok(PingModuleEvents::PostReceivedFilter(decoded));
            }
            if let Ok(decoded) = PostRequestTimeoutReceivedFilter::decode_log(log) {
                return Ok(PingModuleEvents::PostRequestTimeoutReceivedFilter(decoded));
            }
            if let Ok(decoded) = PostResponseReceivedFilter::decode_log(log) {
                return Ok(PingModuleEvents::PostResponseReceivedFilter(decoded));
            }
            if let Ok(decoded) = PostResponseTimeoutReceivedFilter::decode_log(log) {
                return Ok(PingModuleEvents::PostResponseTimeoutReceivedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PingModuleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetResponseReceivedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTimeoutReceivedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MessageDispatchedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PostReceivedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PostRequestTimeoutReceivedFilter(element) =>
                    ::core::fmt::Display::fmt(element, f),
                Self::PostResponseReceivedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PostResponseTimeoutReceivedFilter(element) =>
                    ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetResponseReceivedFilter> for PingModuleEvents {
        fn from(value: GetResponseReceivedFilter) -> Self {
            Self::GetResponseReceivedFilter(value)
        }
    }
    impl ::core::convert::From<GetTimeoutReceivedFilter> for PingModuleEvents {
        fn from(value: GetTimeoutReceivedFilter) -> Self {
            Self::GetTimeoutReceivedFilter(value)
        }
    }
    impl ::core::convert::From<MessageDispatchedFilter> for PingModuleEvents {
        fn from(value: MessageDispatchedFilter) -> Self {
            Self::MessageDispatchedFilter(value)
        }
    }
    impl ::core::convert::From<PostReceivedFilter> for PingModuleEvents {
        fn from(value: PostReceivedFilter) -> Self {
            Self::PostReceivedFilter(value)
        }
    }
    impl ::core::convert::From<PostRequestTimeoutReceivedFilter> for PingModuleEvents {
        fn from(value: PostRequestTimeoutReceivedFilter) -> Self {
            Self::PostRequestTimeoutReceivedFilter(value)
        }
    }
    impl ::core::convert::From<PostResponseReceivedFilter> for PingModuleEvents {
        fn from(value: PostResponseReceivedFilter) -> Self {
            Self::PostResponseReceivedFilter(value)
        }
    }
    impl ::core::convert::From<PostResponseTimeoutReceivedFilter> for PingModuleEvents {
        fn from(value: PostResponseTimeoutReceivedFilter) -> Self {
            Self::PostResponseTimeoutReceivedFilter(value)
        }
    }
    ///Container type for all input parameters for the `dispatch` function with signature
    /// `dispatch((bytes,bytes,uint64,bytes,bytes,uint64,bytes))` and selector `0x70c5474f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dispatch", abi = "dispatch((bytes,bytes,uint64,bytes,bytes,uint64,bytes))")]
    pub struct DispatchCall {
        pub request: GetRequest,
    }
    ///Container type for all input parameters for the `dispatch` function with signature
    /// `dispatch((bytes,bytes,uint64,bytes,uint64,bytes[],uint64))` and selector `0xd5f6eefd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "dispatch",
        abi = "dispatch((bytes,bytes,uint64,bytes,uint64,bytes[],uint64))"
    )]
    pub struct DispatchWithRequestCall {
        pub request: GetRequest,
    }
    ///Container type for all input parameters for the `dispatchPostResponse` function with
    /// signature `dispatchPostResponse(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,
    /// uint64))` and selector `0x4d0d9c3b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "dispatchPostResponse",
        abi = "dispatchPostResponse(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,uint64))"
    )]
    pub struct DispatchPostResponseCall {
        pub response: PostResponse,
    }
    ///Container type for all input parameters for the `dispatchToParachain` function with
    /// signature `dispatchToParachain(uint256)` and selector `0x72354e9b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "dispatchToParachain", abi = "dispatchToParachain(uint256)")]
    pub struct DispatchToParachainCall {
        pub para_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `host` function with signature `host()` and
    /// selector `0xf437bc59`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "host", abi = "host()")]
    pub struct HostCall;
    ///Container type for all input parameters for the `onAccept` function with signature
    /// `onAccept((bytes,bytes,uint64,bytes,bytes,uint64,bytes))` and selector `0xeaee1caa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "onAccept", abi = "onAccept((bytes,bytes,uint64,bytes,bytes,uint64,bytes))")]
    pub struct OnAcceptCall {
        pub request: PostRequest,
    }
    ///Container type for all input parameters for the `onGetResponse` function with signature
    /// `onGetResponse(((bytes,bytes,uint64,bytes,uint64,bytes[],uint64),(bytes,bytes)[]))` and
    /// selector `0xdeae54f5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "onGetResponse",
        abi = "onGetResponse(((bytes,bytes,uint64,bytes,uint64,bytes[],uint64),(bytes,bytes)[]))"
    )]
    pub struct OnGetResponseCall(pub GetResponse);
    ///Container type for all input parameters for the `onGetTimeout` function with signature
    /// `onGetTimeout((bytes,bytes,uint64,bytes,uint64,bytes[],uint64))` and selector `0xc492e426`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "onGetTimeout",
        abi = "onGetTimeout((bytes,bytes,uint64,bytes,uint64,bytes[],uint64))"
    )]
    pub struct OnGetTimeoutCall(pub GetRequest);
    ///Container type for all input parameters for the `onPostRequestTimeout` function with
    /// signature `onPostRequestTimeout((bytes,bytes,uint64,bytes,bytes,uint64,bytes))` and selector
    /// `0xbc0dd447`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "onPostRequestTimeout",
        abi = "onPostRequestTimeout((bytes,bytes,uint64,bytes,bytes,uint64,bytes))"
    )]
    pub struct OnPostRequestTimeoutCall(pub PostRequest);
    ///Container type for all input parameters for the `onPostResponse` function with signature
    /// `onPostResponse(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,uint64))` and selector
    /// `0xfeff7fa8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "onPostResponse",
        abi = "onPostResponse(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,uint64))"
    )]
    pub struct OnPostResponseCall(pub PostResponse);
    ///Container type for all input parameters for the `onPostResponseTimeout` function with
    /// signature `onPostResponseTimeout(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,
    /// uint64))` and selector `0x0bc37bab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "onPostResponseTimeout",
        abi = "onPostResponseTimeout(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,uint64))"
    )]
    pub struct OnPostResponseTimeoutCall(pub PostResponse);
    ///Container type for all input parameters for the `ping` function with signature
    /// `ping((bytes,address,uint64,uint256,uint256))` and selector `0x4a692e06`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "ping", abi = "ping((bytes,address,uint64,uint256,uint256))")]
    pub struct PingCall {
        pub ping_message: PingMessage,
    }
    ///Container type for all input parameters for the `setIsmpHost` function with signature
    /// `setIsmpHost(address)` and selector `0x0e8324a2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setIsmpHost", abi = "setIsmpHost(address)")]
    pub struct SetIsmpHostCall {
        pub host_addr: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PingModuleCalls {
        Dispatch(DispatchCall),
        DispatchWithRequest(DispatchWithRequestCall),
        DispatchPostResponse(DispatchPostResponseCall),
        DispatchToParachain(DispatchToParachainCall),
        Host(HostCall),
        OnAccept(OnAcceptCall),
        OnGetResponse(OnGetResponseCall),
        OnGetTimeout(OnGetTimeoutCall),
        OnPostRequestTimeout(OnPostRequestTimeoutCall),
        OnPostResponse(OnPostResponseCall),
        OnPostResponseTimeout(OnPostResponseTimeoutCall),
        Ping(PingCall),
        SetIsmpHost(SetIsmpHostCall),
    }
    impl ::ethers::core::abi::AbiDecode for PingModuleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DispatchCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Dispatch(decoded));
            }
            if let Ok(decoded) =
                <DispatchWithRequestCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DispatchWithRequest(decoded));
            }
            if let Ok(decoded) =
                <DispatchPostResponseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DispatchPostResponse(decoded));
            }
            if let Ok(decoded) =
                <DispatchToParachainCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DispatchToParachain(decoded));
            }
            if let Ok(decoded) = <HostCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Host(decoded));
            }
            if let Ok(decoded) = <OnAcceptCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnAccept(decoded));
            }
            if let Ok(decoded) = <OnGetResponseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnGetResponse(decoded));
            }
            if let Ok(decoded) = <OnGetTimeoutCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnGetTimeout(decoded));
            }
            if let Ok(decoded) =
                <OnPostRequestTimeoutCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnPostRequestTimeout(decoded));
            }
            if let Ok(decoded) =
                <OnPostResponseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnPostResponse(decoded));
            }
            if let Ok(decoded) =
                <OnPostResponseTimeoutCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnPostResponseTimeout(decoded));
            }
            if let Ok(decoded) = <PingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ping(decoded));
            }
            if let Ok(decoded) = <SetIsmpHostCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetIsmpHost(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PingModuleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Dispatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DispatchWithRequest(element) =>
                    ::ethers::core::abi::AbiEncode::encode(element),
                Self::DispatchPostResponse(element) =>
                    ::ethers::core::abi::AbiEncode::encode(element),
                Self::DispatchToParachain(element) =>
                    ::ethers::core::abi::AbiEncode::encode(element),
                Self::Host(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnAccept(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnGetResponse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnGetTimeout(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnPostRequestTimeout(element) =>
                    ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnPostResponse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnPostResponseTimeout(element) =>
                    ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ping(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetIsmpHost(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PingModuleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Dispatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::DispatchWithRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::DispatchPostResponse(element) => ::core::fmt::Display::fmt(element, f),
                Self::DispatchToParachain(element) => ::core::fmt::Display::fmt(element, f),
                Self::Host(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnAccept(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnGetResponse(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnGetTimeout(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnPostRequestTimeout(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnPostResponse(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnPostResponseTimeout(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ping(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetIsmpHost(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DispatchCall> for PingModuleCalls {
        fn from(value: DispatchCall) -> Self {
            Self::Dispatch(value)
        }
    }
    impl ::core::convert::From<DispatchWithRequestCall> for PingModuleCalls {
        fn from(value: DispatchWithRequestCall) -> Self {
            Self::DispatchWithRequest(value)
        }
    }
    impl ::core::convert::From<DispatchPostResponseCall> for PingModuleCalls {
        fn from(value: DispatchPostResponseCall) -> Self {
            Self::DispatchPostResponse(value)
        }
    }
    impl ::core::convert::From<DispatchToParachainCall> for PingModuleCalls {
        fn from(value: DispatchToParachainCall) -> Self {
            Self::DispatchToParachain(value)
        }
    }
    impl ::core::convert::From<HostCall> for PingModuleCalls {
        fn from(value: HostCall) -> Self {
            Self::Host(value)
        }
    }
    impl ::core::convert::From<OnAcceptCall> for PingModuleCalls {
        fn from(value: OnAcceptCall) -> Self {
            Self::OnAccept(value)
        }
    }
    impl ::core::convert::From<OnGetResponseCall> for PingModuleCalls {
        fn from(value: OnGetResponseCall) -> Self {
            Self::OnGetResponse(value)
        }
    }
    impl ::core::convert::From<OnGetTimeoutCall> for PingModuleCalls {
        fn from(value: OnGetTimeoutCall) -> Self {
            Self::OnGetTimeout(value)
        }
    }
    impl ::core::convert::From<OnPostRequestTimeoutCall> for PingModuleCalls {
        fn from(value: OnPostRequestTimeoutCall) -> Self {
            Self::OnPostRequestTimeout(value)
        }
    }
    impl ::core::convert::From<OnPostResponseCall> for PingModuleCalls {
        fn from(value: OnPostResponseCall) -> Self {
            Self::OnPostResponse(value)
        }
    }
    impl ::core::convert::From<OnPostResponseTimeoutCall> for PingModuleCalls {
        fn from(value: OnPostResponseTimeoutCall) -> Self {
            Self::OnPostResponseTimeout(value)
        }
    }
    impl ::core::convert::From<PingCall> for PingModuleCalls {
        fn from(value: PingCall) -> Self {
            Self::Ping(value)
        }
    }
    impl ::core::convert::From<SetIsmpHostCall> for PingModuleCalls {
        fn from(value: SetIsmpHostCall) -> Self {
            Self::SetIsmpHost(value)
        }
    }
    ///Container type for all return fields from the `dispatch` function with signature
    /// `dispatch((bytes,bytes,uint64,bytes,bytes,uint64,bytes))` and selector `0x70c5474f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DispatchReturn(pub [u8; 32]);
    ///Container type for all return fields from the `dispatch` function with signature
    /// `dispatch((bytes,bytes,uint64,bytes,uint64,bytes[],uint64))` and selector `0xd5f6eefd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DispatchWithRequestReturn(pub [u8; 32]);
    ///Container type for all return fields from the `dispatchPostResponse` function with signature
    /// `dispatchPostResponse(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,uint64))` and
    /// selector `0x4d0d9c3b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DispatchPostResponseReturn(pub [u8; 32]);
    ///Container type for all return fields from the `host` function with signature `host()` and
    /// selector `0xf437bc59`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HostReturn(pub ::ethers::core::types::Address);
    ///`PingMessage(bytes,address,uint64,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PingMessage {
        pub dest: ::ethers::core::types::Bytes,
        pub module: ::ethers::core::types::Address,
        pub timeout: u64,
        pub count: ::ethers::core::types::U256,
        pub fee: ::ethers::core::types::U256,
    }
}
