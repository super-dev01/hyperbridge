/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
  BaseContract,
  BigNumberish,
  BytesLike,
  FunctionFragment,
  Result,
  Interface,
  AddressLike,
  ContractRunner,
  ContractMethod,
  Listener,
} from "ethers";
import type {
  TypedContractEvent,
  TypedDeferredTopicFilter,
  TypedEventLog,
  TypedListener,
  TypedContractMethod,
} from "../common";

export type DispatchGetStruct = {
  dest: BytesLike;
  height: BigNumberish;
  keys: BytesLike[];
  timeout: BigNumberish;
  fee: BigNumberish;
  payer: AddressLike;
};

export type DispatchGetStructOutput = [
  dest: string,
  height: bigint,
  keys: string[],
  timeout: bigint,
  fee: bigint,
  payer: string
] & {
  dest: string;
  height: bigint;
  keys: string[];
  timeout: bigint;
  fee: bigint;
  payer: string;
};

export type PostRequestStruct = {
  source: BytesLike;
  dest: BytesLike;
  nonce: BigNumberish;
  from: BytesLike;
  to: BytesLike;
  timeoutTimestamp: BigNumberish;
  body: BytesLike;
};

export type PostRequestStructOutput = [
  source: string,
  dest: string,
  nonce: bigint,
  from: string,
  to: string,
  timeoutTimestamp: bigint,
  body: string
] & {
  source: string;
  dest: string;
  nonce: bigint;
  from: string;
  to: string;
  timeoutTimestamp: bigint;
  body: string;
};

export type DispatchPostResponseStruct = {
  request: PostRequestStruct;
  response: BytesLike;
  timeout: BigNumberish;
  fee: BigNumberish;
  payer: AddressLike;
};

export type DispatchPostResponseStructOutput = [
  request: PostRequestStructOutput,
  response: string,
  timeout: bigint,
  fee: bigint,
  payer: string
] & {
  request: PostRequestStructOutput;
  response: string;
  timeout: bigint;
  fee: bigint;
  payer: string;
};

export type DispatchPostStruct = {
  dest: BytesLike;
  to: BytesLike;
  body: BytesLike;
  timeout: BigNumberish;
  fee: BigNumberish;
  payer: AddressLike;
};

export type DispatchPostStructOutput = [
  dest: string,
  to: string,
  body: string,
  timeout: bigint,
  fee: bigint,
  payer: string
] & {
  dest: string;
  to: string;
  body: string;
  timeout: bigint;
  fee: bigint;
  payer: string;
};

export interface IDispatcherInterface extends Interface {
  getFunction(
    nameOrSignature:
      | "dispatch((bytes,uint64,bytes[],uint64,uint256,address))"
      | "dispatch(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,uint64,uint256,address))"
      | "dispatch((bytes,bytes,bytes,uint64,uint256,address))"
  ): FunctionFragment;

  encodeFunctionData(
    functionFragment: "dispatch((bytes,uint64,bytes[],uint64,uint256,address))",
    values: [DispatchGetStruct]
  ): string;
  encodeFunctionData(
    functionFragment: "dispatch(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,uint64,uint256,address))",
    values: [DispatchPostResponseStruct]
  ): string;
  encodeFunctionData(
    functionFragment: "dispatch((bytes,bytes,bytes,uint64,uint256,address))",
    values: [DispatchPostStruct]
  ): string;

  decodeFunctionResult(
    functionFragment: "dispatch((bytes,uint64,bytes[],uint64,uint256,address))",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "dispatch(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,uint64,uint256,address))",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "dispatch((bytes,bytes,bytes,uint64,uint256,address))",
    data: BytesLike
  ): Result;
}

export interface IDispatcher extends BaseContract {
  connect(runner?: ContractRunner | null): IDispatcher;
  waitForDeployment(): Promise<this>;

  interface: IDispatcherInterface;

  queryFilter<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>;
  queryFilter<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>;

  on<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    listener: TypedListener<TCEvent>
  ): Promise<this>;
  on<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>;

  once<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    listener: TypedListener<TCEvent>
  ): Promise<this>;
  once<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>;

  listeners<TCEvent extends TypedContractEvent>(
    event: TCEvent
  ): Promise<Array<TypedListener<TCEvent>>>;
  listeners(eventName?: string): Promise<Array<Listener>>;
  removeAllListeners<TCEvent extends TypedContractEvent>(
    event?: TCEvent
  ): Promise<this>;

  "dispatch((bytes,uint64,bytes[],uint64,uint256,address))": TypedContractMethod<
    [request: DispatchGetStruct],
    [string],
    "nonpayable"
  >;

  "dispatch(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,uint64,uint256,address))": TypedContractMethod<
    [response: DispatchPostResponseStruct],
    [string],
    "nonpayable"
  >;

  "dispatch((bytes,bytes,bytes,uint64,uint256,address))": TypedContractMethod<
    [request: DispatchPostStruct],
    [string],
    "nonpayable"
  >;

  getFunction<T extends ContractMethod = ContractMethod>(
    key: string | FunctionFragment
  ): T;

  getFunction(
    nameOrSignature: "dispatch((bytes,uint64,bytes[],uint64,uint256,address))"
  ): TypedContractMethod<[request: DispatchGetStruct], [string], "nonpayable">;
  getFunction(
    nameOrSignature: "dispatch(((bytes,bytes,uint64,bytes,bytes,uint64,bytes),bytes,uint64,uint256,address))"
  ): TypedContractMethod<
    [response: DispatchPostResponseStruct],
    [string],
    "nonpayable"
  >;
  getFunction(
    nameOrSignature: "dispatch((bytes,bytes,bytes,uint64,uint256,address))"
  ): TypedContractMethod<[request: DispatchPostStruct], [string], "nonpayable">;

  filters: {};
}
